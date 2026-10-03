"""
canonical-v2 deterministic JSON canonicalization — implementation revision r2.

Behavioral identity remains canonical-v2.

Historical canonical-v1 and the pre-convergence canonical-v2
implementation remain unchanged and are not routed to this module.
"""

import json
import math
from typing import Any


def canonicalize_v2_r2(obj: Any) -> bytes:
    if obj is None:
        return b"null"

    if isinstance(obj, bool):
        return b"true" if obj else b"false"

    if isinstance(obj, (int, float)):
        return _canonical_number_v2_r2(obj).encode("utf-8")

    if isinstance(obj, str):
        return json.dumps(
            obj,
            ensure_ascii=False,
        ).encode("utf-8")

    if isinstance(obj, list):
        return (
            b"["
            + b",".join(canonicalize_v2_r2(item) for item in obj)
            + b"]"
        )

    if isinstance(obj, dict):
        sorted_keys = sorted(
            key
            for key, value in obj.items()
            if value is not None
        )

        pairs = [
            canonicalize_v2_r2(key)
            + b":"
            + canonicalize_v2_r2(obj[key])
            for key in sorted_keys
        ]

        return b"{" + b",".join(pairs) + b"}"

    raise ValueError(f"Unsupported type: {type(obj)}")


def _canonical_number_v2_r2(value: int | float) -> str:
    try:
        number = float(value)
    except (OverflowError, ValueError) as exc:
        raise ValueError(
            "Number is not representable as finite binary64"
        ) from exc

    if not math.isfinite(number):
        raise ValueError(
            "Non-finite numbers are not valid canonical JSON numbers"
        )

    return _format_ecmascript_number_v2_r2(number)


def _format_ecmascript_number_v2_r2(value: float) -> str:
    if not math.isfinite(value):
        raise ValueError(
            "Non-finite numbers are not valid canonical JSON numbers"
        )

    if value == 0.0:
        return "0"

    raw = repr(value).lower()

    sign = ""
    unsigned = raw

    if unsigned.startswith("-"):
        sign = "-"
        unsigned = unsigned[1:]

    if "e" not in unsigned:
        if unsigned.endswith(".0"):
            unsigned = unsigned[:-2]

        return sign + unsigned

    mantissa, exponent_text = unsigned.split("e", 1)

    exponent = int(exponent_text)

    if "." in mantissa:
        decimal_before = mantissa.index(".")
    else:
        decimal_before = len(mantissa)

    digits = mantissa.replace(".", "").lstrip("0")

    if not digits:
        digits = "0"

    k = exponent + decimal_before
    scientific_exponent = k - 1

    if -6 <= scientific_exponent < 21:
        if k <= 0:
            body = "0." + ("0" * (-k)) + digits
        elif k >= len(digits):
            body = digits + ("0" * (k - len(digits)))
        else:
            body = digits[:k] + "." + digits[k:]

        if "." in body:
            body = body.rstrip("0").rstrip(".")

        return sign + body

    body = digits[0]

    if len(digits) > 1:
        tail = digits[1:].rstrip("0")

        if tail:
            body += "." + tail

    exponent_sign = "+" if scientific_exponent >= 0 else "-"

    return (
        sign
        + body
        + "e"
        + exponent_sign
        + str(abs(scientific_exponent))
    )
