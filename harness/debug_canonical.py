#!/usr/bin/env python3
"""Debug: canonicalize envelope and show SHA-256 for comparison with Go."""

import json
import hashlib
import sys
from pathlib import Path

import harness


def main():
    vector_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("../vectors/core_inv5_001_golden.json")
    
    with open(vector_path, "r", encoding="utf-8") as f:
        vector = json.load(f)
    
    envelope = vector["inputs"]["intent_envelope"]
    
    # Remove signature (same as what signer did)
    envelope_no_sig = {k: v for k, v in envelope.items() if k != "signature"}
    
    # Canonicalize using harness.canonicalize
    canonical = harness.canonicalize(envelope_no_sig)
    
    print("=== Python canonicalization ===")
    print(f"Canonical payload FULL: {canonical.decode('utf-8')}")
    print(f"Canonical payload SHA-256: {hashlib.sha256(canonical).hexdigest()}")
    print(f"Canonical payload length: {len(canonical)}")
    print()
    print(f"Field set: {sorted(envelope_no_sig.keys())}")


if __name__ == "__main__":
    main()