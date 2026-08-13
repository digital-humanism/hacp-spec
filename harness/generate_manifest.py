#!/usr/bin/env python3
"""
Generate conformance manifest with vector digest.

Computes SHA-256 digest over canonicalized vector set for deterministic
verification by conformance harness.
"""

import json
import hashlib
from pathlib import Path
from datetime import datetime

# Import canonicalization from harness
import harness


def generate_manifest(
    vectors_dir: Path,
    output_path: Path,
    spec_version: str = "0.9.2",
    profile: str = "HACP-Core"
):
    """Generate conformance manifest with vector digest."""
    
    if not vectors_dir.exists():
        raise FileNotFoundError(f"Vectors directory not found: {vectors_dir}")

    # Load all vector files, sorted for deterministic ordering
    vector_files = sorted(vectors_dir.glob("*.json"))
    
    if not vector_files:
        raise ValueError(f"No vector files found in {vectors_dir}")

    print(f"Processing {len(vector_files)} vectors from {vectors_dir}")

    # Compute digest over canonicalized vector set
    hasher = hashlib.sha256()
    vector_count = 0
    
    for vf in vector_files:
        print(f"  - {vf.name}")
        with open(vf, "r", encoding="utf-8") as f:
            try:
                # Strict loading to detect duplicate keys
                obj = json.load(f, object_pairs_hook=harness._reject_duplicates)
            except harness.DuplicateKeyError:
                # Include duplicate-key vectors in digest
                with open(vf, "r", encoding="utf-8") as f2:
                    obj = json.load(f2)
            
            # Canonicalize and update hash
            canonical = harness.canonicalize(obj)
            hasher.update(canonical)
            vector_count += 1

    # Build manifest
    vector_digest = f"sha256:{hasher.hexdigest()}"
    
    manifest = {
        "spec_version": spec_version,
        "profile": profile,
        "vector_set": f"core-{spec_version}",
        "canonicalization": "JCS-RFC8785",
        "digest_algorithm": "SHA-256",
        "vector_digest": vector_digest,
        "total_vectors": vector_count,
        "generated_at": datetime.utcnow().isoformat() + "Z"
    }

    # Write manifest
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)

    print(f"\nManifest generated: {output_path}")
    print(f"  Spec version: {spec_version}")
    print(f"  Profile: {profile}")
    print(f"  Vector set: core-{spec_version}")
    print(f"  Total vectors: {vector_count}")
    print(f"  Vector digest: {vector_digest}")


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Generate HACP conformance manifest with vector digest"
    )
    
    parser.add_argument(
        "--vectors-dir",
        type=Path,
        default=Path(__file__).parent.parent / "vectors",
        help="Path to vectors directory"
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(__file__).parent / "conformance_manifest.json",
        help="Output manifest path"
    )
    parser.add_argument(
        "--spec-version",
        default="0.9.2",
        help="HACP specification version"
    )
    parser.add_argument(
        "--profile",
        default="HACP-Core",
        help="Conformance profile"
    )
    
    args = parser.parse_args()
    
    generate_manifest(
        vectors_dir=args.vectors_dir,
        output_path=args.output,
        spec_version=args.spec_version,
        profile=args.profile
    )


if __name__ == "__main__":
    main()