import json
import os
import sys
from pathlib import Path

from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

HARNESS_DIR = Path(__file__).resolve().parents[1]

if str(HARNESS_DIR) not in sys.path:
    sys.path.insert(0, str(HARNESS_DIR))

from canonical_v2_r2 import canonicalize_v2_r2
from harness import compute_sha256, verify_signature


EXPECTED_LENGTH = 423
EXPECTED_SHA256 = (
    "15edb6e1f8cb9124f282a0d1fc54118ace6ee3c43a1d2671553401a20e2c0ed1"
)
EXPECTED_PUBLIC_KEY_HEX = (
    "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3"
)
IMMUTABLE_SIGNATURE = (
    "_5TKPo_xMVHrR-ucaVxKuysTuZaPnoSQgG7o7YVLyzYQW9NlwrjZePdNnlP1D2Vg"
    "AdhwwJDJTLKYN6XP2I45Cw"
)


def test_e13_family2_hash_signature_binding() -> None:
    fixture_path = os.environ.get("HACP_E13_FAMILY2_FIXTURE")
    assert fixture_path, "HACP_E13_FAMILY2_FIXTURE must be set"

    public_key_path = os.environ.get("HACP_E13_FAMILY2_PUBLIC_KEY")
    assert public_key_path, "HACP_E13_FAMILY2_PUBLIC_KEY must be set"

    fixture = json.loads(
        Path(fixture_path).read_text(encoding="utf-8")
    )

    canonical = canonicalize_v2_r2(fixture)

    assert len(canonical) == EXPECTED_LENGTH
    assert compute_sha256(canonical) == EXPECTED_SHA256

    public_key_hex = (
        Path(public_key_path)
        .read_text(encoding="utf-8")
        .strip()
    )

    assert public_key_hex == EXPECTED_PUBLIC_KEY_HEX

    public_key = Ed25519PublicKey.from_public_bytes(
        bytes.fromhex(public_key_hex)
    )

    assert verify_signature(
        canonical,
        IMMUTABLE_SIGNATURE,
        public_key,
    )

    mutated = bytearray(canonical)
    mutated[0] ^= 0x01
    mutated_bytes = bytes(mutated)

    changed = sum(
        left != right
        for left, right in zip(canonical, mutated_bytes)
    )

    assert changed == 1

    assert not verify_signature(
        mutated_bytes,
        IMMUTABLE_SIGNATURE,
        public_key,
    )