"""
canonical-v2 successor surface for Python.

Historical canonical-v1 implementations remain frozen in:
- harness/harness.py
- tools/bake_vector.py
- tools/enforcement_v2_bake_vectors.py

This module is additive only and is not routed into existing runtime,
conformance, baking, hashing, or signing paths.
"""

import json
import math
from typing import Any


def canonicalize_v2(obj: Any) -> bytes:
    if obj is None:
        return b"null"

    if isinstance(obj, bool):
        return b"true" if obj else b"false"

    if isinstance(obj, int):
        return str(obj).encode("utf-8")

    if isinstance(obj, float):
        if not math.isfinite(obj):
            raise ValueError(
                "Non-finite numbers are not valid canonical JSON numbers"
            )

        return json.dumps(
            obj,
            ensure_ascii=False,
            allow_nan=False,
            separators=(",", ":"),
        ).encode("utf-8")

    if isinstance(obj, str):
        return json.dumps(
            obj,
            ensure_ascii=False,
        ).encode("utf-8")

    if isinstance(obj, list):
        return (
            b"["
            + b",".join(canonicalize_v2(item) for item in obj)
            + b"]"
        )

    if isinstance(obj, dict):
        sorted_keys = sorted(
            key
            for key, value in obj.items()
            if value is not None
        )

        pairs = [
            canonicalize_v2(key)
            + b":"
            + canonicalize_v2(obj[key])
            for key in sorted_keys
        ]

        return b"{" + b",".join(pairs) + b"}"

    raise ValueError(f"Unsupported type: {type(obj)}")
