package main

import (
	"crypto/ed25519"
	"encoding/json"
	"strings"
)

func getStr(m map[string]interface{}, key string) string {
	if v, ok := m[key].(string); ok {
		return v
	}
	return ""
}

func getMap(m map[string]interface{}, key string) map[string]interface{} {
	if v, ok := m[key].(map[string]interface{}); ok {
		return v
	}
	return nil
}

func getInt(m map[string]interface{}, key string) (int64, bool) {
	if v, ok := m[key].(json.Number); ok {
		if i, err := v.Int64(); err == nil {
			return i, true
		}
	}
	return 0, false
}

func inStringList(list []interface{}, val string) bool {
	for _, item := range list {
		if s, ok := item.(string); ok && s == val {
			return true
		}
	}
	return false
}

// Evaluate implements the HACP evaluate() policy logic per spec.
// Returns ALLOW, DENY, or CHECKPOINT.
func Evaluate(action, envelope, context map[string]interface{}, token map[string]interface{}) string {
	// Clock
	currentTime, ok := getInt(context, "current_time")
	if !ok {
		currentTime, ok = getInt(context, "clock")
	}
	if !ok {
		currentTime, _ = getInt(envelope, "issued_at")
	}

	// Envelope expiry
	if exp, ok := getInt(envelope, "expires_at"); ok && currentTime > exp {
		return "DENY"
	}

	// Envelope revocation
	if rev, ok := context["revoked_envelopes"].([]interface{}); ok {
		if inStringList(rev, getStr(envelope, "envelope_id")) {
			return "DENY"
		}
	}

	// Token revocation + expiry
	if token != nil {
		if rev, ok := context["revoked_tokens"].([]interface{}); ok {
			if inStringList(rev, getStr(token, "token_id")) {
				return "DENY"
			}
		}
		if exp, ok := getInt(token, "expires_at"); ok && currentTime > exp {
			return "DENY"
		}
	}

	// Trusted keys
	if trusted, ok := context["trusted_keys"].([]interface{}); ok {
		if !inStringList(trusted, getStr(envelope, "signer_key_id")) {
			return "DENY"
		}
		if token != nil && !inStringList(trusted, getStr(token, "signer_key_id")) {
			return "DENY"
		}
	}

	// Reject non-Ed25519 algorithms
	if strings.Contains(strings.ToLower(getStr(envelope, "signer_key_id")), "hmac") {
		return "DENY"
	}
	if token != nil && strings.Contains(strings.ToLower(getStr(token, "signer_key_id")), "hmac") {
		return "DENY"
	}

	// Bounded autonomy (INV-7)
	if budget := getMap(envelope, "autonomy_budget"); budget != nil {
		if maxActions, ok := getInt(budget, "max_actions"); ok {
			current, _ := getInt(context, "current_action_count")
			if current >= maxActions {
				return "DENY"
			}
		}
	}

	// Human final decision (INV-1)
	if humanVerbs, ok := context["human_required_verbs"].([]interface{}); ok {
		if inStringList(humanVerbs, getStr(action, "verb")) {
			if getStr(envelope, "principal_kind") == "system" {
				if getStr(envelope, "parent_envelope_id") == "" {
					return "CHECKPOINT"
				}
			}
		}
	}

	// Boundary re-authorization (INV-2)
	scope := getMap(envelope, "scope")
	if scope == nil {
		return "DENY"
	}

	checks := []struct {
		attr string
		key  string
	}{
		{"audience", "audiences"},
		{"reversibility", "reversibility"},
		{"externality", "externality"},
		{"data_class", "data_classes"},
		{"verb", "verbs"},
		{"resource_class", "resource_classes"},
	}
	for _, c := range checks {
		if allowed, ok := scope[c.key].([]interface{}); ok {
			if !inStringList(allowed, getStr(action, c.attr)) {
				return "DENY"
			}
		}
	}

	// Quantity
	if q, ok := getInt(action, "quantity"); ok {
		if maxQ, ok2 := getInt(scope, "max_quantity"); ok2 && q > maxQ {
			return "DENY"
		}
	}

	// Destination
	if dest := getStr(action, "destination"); dest != "" {
		if allowed, ok := scope["destinations"].([]interface{}); ok && len(allowed) > 0 {
			if !inStringList(allowed, dest) {
				return "DENY"
			}
		}
	}

	// Tool name
	if tool := getStr(action, "tool_name"); tool != "" {
		if allowed, ok := scope["tool_names"].([]interface{}); ok && len(allowed) > 0 {
			if !inStringList(allowed, tool) {
				return "DENY"
			}
		}
	}

	return "ALLOW"
}

const genesisHash = "0000000000000000000000000000000000000000000000000000000000000000"

// verifyProvenance checks payload hash, linkage, and signature.
func verifyProvenance(event, prior map[string]interface{}, pub ed25519.PublicKey) bool {
	payload, ok := event["payload"]
	if !ok {
		return false
	}
	pb, err := Canonicalize(payload)
	if err != nil {
		return false
	}
	if SHA256Hex(pb) != getStr(event, "payload_hash") {
		return false
	}

	expectedPrev := genesisHash
	if prior != nil {
		prb, err := Canonicalize(prior)
		if err != nil {
			return false
		}
		expectedPrev = SHA256Hex(prb)
	}
	if getStr(event, "prev_event_hash") != expectedPrev {
		return false
	}

	evNoSig := copyWithout(event, "signature")
	evb, err := Canonicalize(evNoSig)
	if err != nil {
		return false
	}
	return VerifySignature(pub, evb, getStr(event, "signature"))
}
