import hashlib
import json
import os
import struct
import sys
from pathlib import Path

HARNESS_DIR = Path(__file__).resolve().parents[1]

if str(HARNESS_DIR) not in sys.path:
    sys.path.insert(0, str(HARNESS_DIR))

from canonical_v2_r2 import canonicalize_v2_r2

EXPECTED_LENGTH = 1642
EXPECTED_SHA256 = (
    "45c486b9ad1cb7a7a958f9876eb41c474cb62f144ee2ef947872c13a4ea91bb7"
)


def _load_e13_manifest():
    manifest_path = os.environ.get("HACP_E13_CORPUS")

    assert manifest_path, (
        "HACP_E13_CORPUS must be set to the absolute E13 manifest path"
    )

    data = Path(manifest_path).read_bytes()

    assert len(data) == EXPECTED_LENGTH, (
        f"E13 corpus manifest byte length mismatch: "
        f"expected {EXPECTED_LENGTH}, got {len(data)}"
    )

    actual_hash = hashlib.sha256(data).hexdigest()

    assert actual_hash == EXPECTED_SHA256, (
        f"E13 corpus manifest SHA-256 mismatch: "
        f"expected {EXPECTED_SHA256}, got {actual_hash}"
    )

    rows = json.loads(data.decode("utf-8"))

    assert isinstance(rows, list)
    assert len(rows) == 24

    expected_ids = [f"R{i:02d}" for i in range(1, 25)]
    actual_ids = [row["id"] for row in rows]

    assert actual_ids == expected_ids

    return rows


def test_e13_canonical_v2_r01_r24_matches_shared_manifest():
    rows = _load_e13_manifest()

    for row in rows:
        assert len(row["bits"]) == 16

        bits = int(row["bits"], 16)

        value = struct.unpack(
            ">d",
            bits.to_bytes(8, byteorder="big"),
        )[0]

        actual = canonicalize_v2_r2(value)

        assert actual == row["expected"].encode("utf-8"), (
            f'{row["id"]} canonical-v2 mismatch for bits {row["bits"]}: '
            f'expected {row["expected"]!r}, got {actual.decode("utf-8")!r}'
        )