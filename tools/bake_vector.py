#!/usr/bin/env python3
"""
HACP Vector Baker — Deterministic vector generation tool.

Bakes cryptographic artifacts (action_hash, signatures) into conformance
vectors using the fixed test keypair. This tool is NOT part of the
conformance path — it runs offline to produce reproducible vectors.

Usage:
    python tools/bake_vector.py              # Bake all golden vectors
    python tools/bake_vector.py --check      # Verify vectors match (CI mode)
    python tools/bake_vector.py --vector vectors/core_inv3_001_golden.json

Rules:
    - Golden vectors: computes action_hash, signs envelope and token
    - Negative vectors: untouched (they contain intentionally broken data)
    - draft_mode: false after baking
    - signer_key_id: key-ed25519-test-001
    - policy_context.clock: explicit, no time.time()
"""

import argparse
import json
import hashlib
import sys
from pathlib import Path
from typing import Dict, Any, Optional

from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives.serialization import Encoding, PublicFormat

# --- Configuration ---
SPEC_ROOT = Path(__file__).parent.parent
VECTORS_DIR = SPEC_ROOT / "vectors"
KEYS_DIR = SPEC_ROOT / "harness" / "keys"
SEED_FILE = KEYS_DIR / "test-ed25519-001.seed"

SIGNER_KEY_ID = "key-ed25519-test-001"

# Signatures that indicate "not yet baked" and should be replaced
PLACEHOLDER_SIGNATURES = (
    "PLACEHOLDER",
    "dummy",
    "",
    "REPLACE_WITH_VALID_ED25519_SIG_BASE64URL",
    "HMAC_SIGNATURE_PLACEHOLDER",
    "HMAC_TOKEN_SIGNATURE_PLACEHOLDER",
    "TAMPERED_SIGNATURE_1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ",
)

# --- Canonicalization (JCS-like, RFC 8785) ---
def canonicalize(obj: Any) -> bytes:
    """
    Strict JCS-like canonicalization for deterministic serialization.
    Conforms to RFC 8785 (JSON Canonicalization Scheme).
    """
    if obj is None:
        return b"null"
    if isinstance(obj, bool):
        return b"true" if obj else b"false"
    if isinstance(obj, int):
        return str(obj).encode("utf-8")
    if isinstance(obj, str):
        return json.dumps(obj, ensure_ascii=False).encode("utf-8")
    if isinstance(obj, list):
        return b"[" + b",".join(canonicalize(item) for item in obj) + b"]"
    if isinstance(obj, dict):
        sorted_keys = sorted(obj.keys())
        pairs = [canonicalize(k) + b":" + canonicalize(obj[k]) for k in sorted_keys]
        return b"{" + b",".join(pairs) + b"}"
    raise ValueError(f"Unsupported type: {type(obj)}")


def compute_sha256(data: bytes) -> str:
    """Compute SHA-256 hash as lowercase hex string."""
    return hashlib.sha256(data).hexdigest()


# --- Key Loading ---
def load_signing_key() -> Ed25519PrivateKey:
    """Load Ed25519 private key from fixed seed."""
    if not SEED_FILE.exists():
        print(f"Error: seed file not found at {SEED_FILE}")
        print("Run 'python tools/gen_test_keys.py' first.")
        sys.exit(1)

    seed_hex = SEED_FILE.read_text(encoding="utf-8").strip()
    seed = bytes.fromhex(seed_hex)

    if len(seed) != 32:
        print(f"Error: seed must be 32 bytes, got {len(seed)}")
        sys.exit(1)

    return Ed25519PrivateKey.from_private_bytes(seed)


def get_public_key_hex(private_key: Ed25519PrivateKey) -> str:
    """Get public key as hex string."""
    pk = private_key.public_key()
    pk_bytes = pk.public_bytes(Encoding.Raw, PublicFormat.Raw)
    return pk_bytes.hex()


# --- Signing ---
def sign_payload(payload_bytes: bytes, private_key: Ed25519PrivateKey) -> str:
    """Sign payload and return Base64url encoded signature (no padding)."""
    import base64
    signature = private_key.sign(payload_bytes)
    return base64.urlsafe_b64encode(signature).rstrip(b"=").decode("utf-8")


# --- Vector Baking ---
def bake_vector(vector: Dict, private_key: Ed25519PrivateKey) -> Dict:
    """
    Bake cryptographic artifacts into a golden vector.

    For golden vectors:
    - Computes action_hash from canonicalized proposed_action
    - Signs intent_envelope (replaces 'dummy' signature)
    - Signs decision_token (replaces 'PLACEHOLDER' signature)
    - Sets draft_mode to false
    - Sets signer_key_id to test key
    - Ensures policy_context.clock is set

    For negative vectors: returns unchanged.
    """
    test_type = vector.get("type")

    # Negative vectors are untouched
    if test_type != "golden":
        return vector

    action = vector["inputs"]["proposed_action"]

    # Step 1: Compute action_hash
    canonical_action = canonicalize(action)
    action_hash = compute_sha256(canonical_action)

    # Step 2: Sign intent_envelope
    envelope = vector["inputs"].get("intent_envelope")
    if envelope:
        envelope["signer_key_id"] = SIGNER_KEY_ID
        if envelope.get("signature") in PLACEHOLDER_SIGNATURES:
            envelope_for_signing = {
                k: v for k, v in envelope.items() if k != "signature"
            }
            envelope["signature"] = sign_payload(
                canonicalize(envelope_for_signing), private_key
            )

    # Step 3: Sign decision_token
    token = vector["inputs"].get("decision_token")
    if token:
        token["action_hash"] = action_hash
        token["signer_key_id"] = SIGNER_KEY_ID
        if token.get("signature") in PLACEHOLDER_SIGNATURES:
            token_for_signing = {
                k: v for k, v in token.items() if k != "signature"
            }
            token["signature"] = sign_payload(
                canonicalize(token_for_signing), private_key
            )

    # Step 4: Ensure policy_context.clock is set
    context = vector.get("policy_context", {})
    if "clock" not in context:
        # Default clock: proposed_at or issued_at
        clock = action.get("proposed_at", envelope.get("issued_at", 0) if envelope else 0)
        context["clock"] = clock
    vector["policy_context"] = context

    # Step 5: Set draft_mode to false
    vector["draft_mode"] = False

    # Step 6: Add expected action_hash for verifier
    if "expected" not in vector:
        vector["expected"] = {}
    vector["expected"]["action_hash"] = action_hash

    return vector


def verify_vector(vector: Dict, private_key: Ed25519PrivateKey) -> bool:
    """
    Verify that a baked vector's cryptographic artifacts are correct.
    Returns True if valid, False otherwise.
    """
    test_type = vector.get("type")
    if test_type != "golden":
        return True  # Negative vectors are not verified

    action = vector["inputs"]["proposed_action"]
    canonical_action = canonicalize(action)
    expected_hash = compute_sha256(canonical_action)

    # Check action_hash
    token = vector["inputs"].get("decision_token")
    if token:
        if token.get("action_hash") != expected_hash:
            print(f"  [FAIL] action_hash mismatch")
            return False

        # Verify signature
        token_for_verify = {
            k: v for k, v in token.items() if k != "signature"
        }
        import base64
        try:
            sig_b64 = token["signature"]
            sig_bytes = base64.urlsafe_b64decode(sig_b64 + "=" * (-len(sig_b64) % 4))
            public_key = private_key.public_key()
            public_key.verify(sig_bytes, canonicalize(token_for_verify))
        except Exception as e:
            print(f"  [FAIL] signature verification: {e}")
            return False

    return True


# --- Main ---
def main():
    parser = argparse.ArgumentParser(
        description="HACP Vector Baker - Deterministic vector generation"
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Verify vectors match expected hashes/signatures (CI mode)"
    )
    parser.add_argument(
        "--vector",
        type=Path,
        help="Bake a single vector file"
    )
    parser.add_argument(
        "--vectors-dir",
        type=Path,
        default=VECTORS_DIR,
        help="Path to vectors directory"
    )

    args = parser.parse_args()

    # Load signing key
    private_key = load_signing_key()
    public_hex = get_public_key_hex(private_key)
    print(f"Loaded test key: {SIGNER_KEY_ID}")
    print(f"Public key: {public_hex}")
    print()

    # Determine files to process
    if args.vector:
        v_path = Path(args.vector)
        if not v_path.is_absolute():
            v_path = SPEC_ROOT / v_path
        vector_files = [v_path]

    else:
        vector_files = sorted(args.vectors_dir.glob("*.json"))

    if not vector_files:
        print("No vector files found.")
        sys.exit(0)

    # Process vectors
    baked_count = 0
    skipped_count = 0
    failed_count = 0

    for v_file in vector_files:
        with open(v_file, "r", encoding="utf-8") as f:
            vector = json.load(f)

        test_id = vector.get("test_id", v_file.stem)
        test_type = vector.get("type", "unknown")

        if args.check:
            # Verify mode
            print(f"[CHECK] {test_id} ({test_type})", end=" ")
            if verify_vector(vector, private_key):
                print("OK")
            else:
                print("FAILED")
                failed_count += 1
        else:
            # Bake mode
            if test_type == "golden":
                print(f"[BAKE]  {test_id}", end=" ")
                baked = bake_vector(vector, private_key)

                # Write back
                with open(v_file, "w", encoding="utf-8") as f:
                    json.dump(baked, f, indent=2, ensure_ascii=False)
                    f.write("\n")

                print("done")
                baked_count += 1
            else:
                print(f"[SKIP]  {test_id} ({test_type})")
                skipped_count += 1

    # Summary
    print()
    print("=" * 60)
    if args.check:
        total = len(vector_files)
        passed = total - failed_count
        print(f"CHECK RESULTS: {passed}/{total} passed")
        if failed_count > 0:
            sys.exit(1)
    else:
        print(f"BAKE RESULTS: {baked_count} baked, {skipped_count} skipped")
    print("=" * 60)


if __name__ == "__main__":
    main()