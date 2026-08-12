package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"strings"
)

// testPubKeyHex is the published conformance test public key (harness/keys/KEYS.md).
// Embedded so the binary is CWD-independent. TEST ONLY.
const testPubKeyHex = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3"

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, `{"error":"USAGE","message":"usage: hacp-go evaluate --vector <path>"}`)
		os.Exit(2)
	}

	switch os.Args[1] {
	case "evaluate":
		fs := flag.NewFlagSet("evaluate", flag.ExitOnError)
		vectorPath := fs.String("vector", "", "path to vector JSON")
		pubKeyPath := fs.String("public-key", "", "optional path to public key (hex)")
		fs.Parse(os.Args[2:])

		if *vectorPath == "" {
			fmt.Fprintln(os.Stderr, `{"error":"USAGE","message":"--vector required"}`)
			os.Exit(2)
		}
		os.Exit(runEvaluate(*vectorPath, *pubKeyPath))
	default:
		fmt.Fprintln(os.Stderr, `{"error":"USAGE","message":"unknown command"}`)
		os.Exit(2)
	}
}

func runEvaluate(vectorPath, pubKeyPath string) int {
	data, err := os.ReadFile(vectorPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, errJSON("READ_ERROR", err.Error()))
		return 1
	}

	var root map[string]interface{}
	dec := json.NewDecoder(bytes.NewReader(data))
	dec.UseNumber()
	if err := dec.Decode(&root); err != nil {
		fmt.Fprintln(os.Stderr, errJSON("PARSE_ERROR", err.Error()))
		return 1
	}

	inputs, _ := root["inputs"].(map[string]interface{})
	action, _ := inputs["proposed_action"].(map[string]interface{})
	envelope, _ := inputs["intent_envelope"].(map[string]interface{})
	context, _ := root["policy_context"].(map[string]interface{})
	token, _ := inputs["decision_token"].(map[string]interface{})
	if context == nil {
		context = map[string]interface{}{}
	}

	// Load public key (embedded default or override)
	pubHex := testPubKeyHex
	if pubKeyPath != "" {
		if b, err := os.ReadFile(pubKeyPath); err == nil {
			pubHex = strings.TrimSpace(string(b))
		}
	}
	pub, err := LoadPublicKey(pubHex)
	if err != nil {
		fmt.Fprintln(os.Stderr, errJSON("KEY_ERROR", err.Error()))
		return 1
	}

	// Runtime checkpoint pre-step
	if cp, ok := inputs["checkpoint"].(map[string]interface{}); ok {
		if d := checkpointDecision(cp, context); d != nil {
			out, _ := json.Marshal(map[string]interface{}{"decision": *d})
			fmt.Println(string(out))
			return 0
		}
	}

	// Policy evaluation
	decision := Evaluate(action, envelope, context, token)

	// Crypto verification (INV-3, INV-5)
	if decision == "ALLOW" && token != nil {
		canonicalAction, err := Canonicalize(action)
		if err != nil {
			decision = "DENY"
		} else {
			computed := SHA256Hex(canonicalAction)
			if getStr(token, "action_hash") != computed {
				decision = "DENY"
			} else {
				tokenNoSig := copyWithout(token, "signature")
				payload, _ := Canonicalize(tokenNoSig)
				if !VerifySignature(pub, payload, getStr(token, "signature")) {
					decision = "DENY"
				}
			}
		}
	}

	// Provenance verification (INV-4)
	provenance, _ := inputs["provenance_event"].(map[string]interface{})
	prior, _ := inputs["prior_provenance_event"].(map[string]interface{})
	omit, _ := inputs["omit_provenance"].(bool)

	if decision == "ALLOW" {
		if omit {
			decision = "DENY"
		} else if provenance != nil {
			if !verifyProvenance(provenance, prior, pub) {
				decision = "DENY"
			}
		}
	}

	// Build AgencyDecision response
	resp := map[string]interface{}{"decision": decision}
	if decision == "ALLOW" && token != nil {
		resp["decision_token"] = token
	}
	if decision == "ALLOW" && provenance != nil && !omit {
		resp["provenance_event_id"] = provenance["event_id"]
	}

	out, _ := json.Marshal(resp)
	fmt.Println(string(out))
	return 0
}

func copyWithout(m map[string]interface{}, key string) map[string]interface{} {
	out := make(map[string]interface{}, len(m))
	for k, v := range m {
		if k != key {
			out[k] = v
		}
	}
	return out
}

func errJSON(code, msg string) string {
	b, _ := json.Marshal(map[string]string{"error": code, "message": msg})
	return string(b)
}
