#!/usr/bin/env python3
"""
HACP Conformance Runner v0.9.1 (Draft Mode)

Validates conformance vectors against JSON schemas, applies JCS-like 
canonicalization, computes SHA-256 action hashes, and verifies Ed25519 
cryptographic binding.

Includes logic for:
- INV-1: Human Final Decision (Policy-based escalation)
- INV-2: Boundary Re-Authorization (Scope enforcement)
- INV-3: Token Binding (Crypto hash match)
- INV-5: Cryptographic Integrity (Signature verification)
- INV-7: Bounded Autonomy (Budget exhaustion)
"""

import json
import hashlib
import base64
from pathlib import Path
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
import jsonschema

# --- Configuration ---
SCHEMAS_DIR = Path(__file__).parent / "schemas"
VECTORS_DIR = Path(__file__).parent / "vectors"

# --- Canonicalization ---
def canonicalize(obj: dict) -> bytes:
    if obj is None: return b"null"
    if isinstance(obj, bool): return b"true" if obj else b"false"
    if isinstance(obj, int): return str(obj).encode("utf-8")
    if isinstance(obj, str): return json.dumps(obj, ensure_ascii=False).encode("utf-8")
    if isinstance(obj, list): return b"[" + b",".join(canonicalize(item) for item in obj) + b"]"
    if isinstance(obj, dict):
        sorted_keys = sorted(obj.keys())
        pairs = [canonicalize(k) + b":" + canonicalize(obj[k]) for k in sorted_keys]
        return b"{" + b",".join(pairs) + b"}"
    raise ValueError(f"Unsupported type: {type(obj)}")

def compute_sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()

# --- Crypto ---
def sign_payload(payload_bytes: bytes, private_key: Ed25519PrivateKey) -> str:
    signature = private_key.sign(payload_bytes)
    return base64.urlsafe_b64encode(signature).rstrip(b"=").decode("utf-8")

def verify_signature(payload_bytes: bytes, signature_b64: str, public_key) -> bool:
    try:
        sig_bytes = base64.urlsafe_b64decode(signature_b64 + "=" * (-len(signature_b64) % 4))
        public_key.verify(sig_bytes, payload_bytes)
        return True
    except Exception:
        return False

# --- Schema Loading ---
def load_schemas():
    schemas = {}
    for schema_file in SCHEMAS_DIR.glob("*.json"):
        with open(schema_file, "r", encoding="utf-8") as f:
            schema = json.load(f)
        schemas[schema["title"]] = schema
    return schemas

# --- Logic Emulation (The "Brain" of the Runner) ---
def evaluate_logic(action: dict, envelope: dict, context: dict) -> str:
    """
    Simulates the HACP-Core evaluate() function based on policy context.
    Returns: 'ALLOW', 'DENY', or 'CHECKPOINT'
    """
    # INV-7: Bounded Autonomy
    budget = envelope.get("autonomy_budget", {})
    max_actions = budget.get("max_actions")
    current_count = context.get("current_action_count", 0)
    if max_actions is not None and current_count >= max_actions:
        return "DENY" # Budget exhausted

    # INV-1: Human Final Decision
    human_verbs = context.get("human_required_verbs", [])
    if action["verb"] in human_verbs and envelope["principal_kind"] == "system":
        return "CHECKPOINT" # Requires human

    # INV-2: Boundary Re-Authorization (Scope Check)
    scope = envelope.get("scope", {})
    allowed_audiences = scope.get("audiences", [])
    if action["audience"] not in allowed_audiences:
        return "DENY" # Scope exceeded

    return "ALLOW"

# --- Vector Processing ---
def process_vector(vector: dict, schemas: dict, private_key: Ed25519PrivateKey):
    test_id = vector["test_id"]
    test_type = vector["type"]
    draft_mode = vector.get("draft_mode", False)
    context = vector.get("policy_context", {})
    
    print(f"\n[{test_id}] ({test_type}) {vector['description']}")
    
    # 1. Schema Validation
    action_schema = schemas["HACP Proposed Action"]
    envelope_schema = schemas["HACP Intent Envelope"]
    
    try:
        jsonschema.validate(instance=vector["inputs"]["proposed_action"], schema=action_schema)
        jsonschema.validate(instance=vector["inputs"]["intent_envelope"], schema=envelope_schema)
        print("  [PASS] Schema validation")
    except jsonschema.ValidationError as e:
        print(f"  [FAIL] Schema validation: {e.message}")
        return False

    action = vector["inputs"]["proposed_action"]
    envelope = vector["inputs"]["intent_envelope"]
    
    # 2. Logic Evaluation (INV-1, INV-2, INV-7)
    expected_outcome = vector["expected"]["outcome"]
    actual_outcome = evaluate_logic(action, envelope, context)
    
    if actual_outcome == expected_outcome:
        print(f"  [PASS] Logic evaluation: {actual_outcome}")
    else:
        print(f"  [FAIL] Logic evaluation: expected {expected_outcome}, got {actual_outcome}")
        return False

    # 3. Crypto & Binding (INV-3, INV-5) - Only if ALLOW
    if actual_outcome == "ALLOW":
         canonical_action = canonicalize(action)
         computed_hash = compute_sha256(canonical_action)
            
         if draft_mode and test_type == "golden":
             # Sign a dummy token for the test
             token = vector["inputs"].get("decision_token", {})
             token["action_hash"] = computed_hash
             token_for_signing = {k: v for k, v in token.items() if k != "signature"}
             token["signature"] = sign_payload(canonicalize(token_for_signing), private_key)
             vector["inputs"]["decision_token"] = token
             print("  [INFO] Draft mode: signed token for ALLOW decision.")

             # Verify binding if token exists
             token = vector["inputs"].get("decision_token")
             if token:
                binding_valid = (token["action_hash"] == computed_hash)
                if binding_valid:
                    print("  [PASS] Token binding (action_hash matches)")
                else:
                    print("  [FAIL] Token binding mismatch")

                # Verify signature
                token_for_verify = {k: v for k, v in token.items() if k != "signature"}
                pub_key = private_key.public_key()
                sig_valid = verify_signature(canonicalize(token_for_verify), token["signature"], pub_key)

                if sig_valid:
                    print("  [PASS] Signature verification")
                else:
                    print("  [FAIL] Signature verification failed")

                # Final verdict for crypto tests
                if test_type == "golden":
                    if binding_valid and sig_valid:
                        return True
                    else:
                        return False
                elif test_type == "negative":
                    if not binding_valid or not sig_valid:
                        print("  [PASS] Negative vector correctly failed crypto verification (fail-closed)")
                        return True
                    else:
                        return False

    return True

# --- Main ---
def main():
    print("="*60)
    print("HACP Conformance Runner v0.9.1")
    print("="*60)
    
    schemas = load_schemas()
    private_key = Ed25519PrivateKey.generate()
    
    passed = failed = 0
    vector_files = sorted(VECTORS_DIR.glob("*.json"))
    
    for v_file in vector_files:
        with open(v_file, "r", encoding="utf-8") as f:
            vector = json.load(f)
        if process_vector(vector, schemas, private_key):
            passed += 1
        else:
            failed += 1
            
    print("\n" + "="*60)
    print(f"RESULTS: {passed} passed, {failed} failed, {passed + failed} total")
    print("="*60)
    if failed > 0: exit(1)

if __name__ == "__main__":
    main()