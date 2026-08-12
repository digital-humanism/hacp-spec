import * as fs from "fs";
import type { KeyObject } from "crypto";
import * as crypto from "crypto";
import { canonicalBytes } from "./canonical";
import { sha256Hex, loadPublicKey, verifySignature } from "./crypto";
import { evaluate } from "./evaluate";

// Published conformance test public key (harness/keys/KEYS.md). TEST ONLY.
const TEST_PUB_HEX =
  "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

const GENESIS = "0".repeat(64);

function verifyProvenance(event: any, prior: any, pub: KeyObject): boolean {
  if (sha256Hex(canonicalBytes(event.payload)) !== event.payload_hash) return false;
  const expectedPrev = prior ? sha256Hex(canonicalBytes(prior)) : GENESIS;
  if (event.prev_event_hash !== expectedPrev) return false;
  const { signature, ...evNoSig } = event;
  return verifySignature(pub, canonicalBytes(evNoSig), signature);
}

function main(): void {
  const args = process.argv.slice(2);
  if (args[0] !== "evaluate") {
    console.error(
      JSON.stringify({
        error: "USAGE",
        message: "usage: hacp-ts evaluate --vector <path>",
      })
    );
    process.exit(2);
  }

  let vectorPath = "";
  let pubKeyPath = "";
  for (let i = 1; i < args.length; i++) {
    if (args[i] === "--vector") {
      vectorPath = args[++i] ?? "";
    } else if (args[i] === "--public-key") {
      pubKeyPath = args[++i] ?? "";
    }
  }

  if (!vectorPath) {
    console.error(
      JSON.stringify({ error: "USAGE", message: "--vector required" })
    );
    process.exit(2);
  }

  process.exit(runEvaluate(vectorPath, pubKeyPath));
}

function runEvaluate(vectorPath: string, pubKeyPath: string): number {
  let root: any;
  try {
    root = JSON.parse(fs.readFileSync(vectorPath, "utf-8"));
  } catch (e: any) {
    console.error(
      JSON.stringify({ error: "PARSE_ERROR", message: e.message })
    );
    return 1;
  }

  const inputs = root.inputs ?? {};
  const action = inputs.proposed_action ?? {};
  const envelope = inputs.intent_envelope ?? {};
  const context = root.policy_context ?? {};
  const token = inputs.decision_token ?? null;

  let pubHex = TEST_PUB_HEX;
  if (pubKeyPath) {
    try {
      pubHex = fs.readFileSync(pubKeyPath, "utf-8").trim();
    } catch {
      // fallback to the published test key
    }
  }

  let pub: KeyObject;
  try {
    pub = loadPublicKey(pubHex);
  } catch (e: any) {
    console.error(
      JSON.stringify({ error: "KEY_ERROR", message: e.message })
    );
    return 1;
  }

  let decision = evaluate(action, envelope, context, token);

  // Crypto verification (INV-3, INV-5)
  if (decision === "ALLOW" && token) {
    const computed = sha256Hex(canonicalBytes(action));
    if (token.action_hash !== computed) {
      decision = "DENY";
    } else {
      const { signature, ...tokenNoSig } = token;
      if (!verifySignature(pub, canonicalBytes(tokenNoSig), signature)) {
        decision = "DENY";
      }
    }
  }
 
  const provenance = inputs.provenance_event ?? null;
  const prior = inputs.prior_provenance_event ?? null;
  const omit = inputs.omit_provenance === true;

  if (decision === "ALLOW") {
    if (omit) {
      decision = "DENY";
    } else if (provenance && !verifyProvenance(provenance, prior, pub)) {
      decision = "DENY";
    }
  }
 
  const resp: any = { decision };
  if (decision === "ALLOW" && token) {
    resp.decision_token = token;
  }
  if (decision === "ALLOW" && provenance && !omit) {
  resp.provenance_event_id = provenance.event_id;
  }
  console.log(JSON.stringify(resp));
  return 0;
}

main();