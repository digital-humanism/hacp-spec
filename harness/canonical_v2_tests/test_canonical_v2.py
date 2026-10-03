import math
import sys
from pathlib import Path

import pytest


HARNESS_DIR = Path(__file__).resolve().parents[1]

if str(HARNESS_DIR) not in sys.path:
    sys.path.insert(0, str(HARNESS_DIR))

from canonical_v2 import canonicalize_v2


def test_f1_finite_fractional_number_is_preserved():
    assert canonicalize_v2(
        {"constraints": {"x": 1.5}}
    ) == b'{"constraints":{"x":1.5}}'


def test_f2_null_valued_object_member_is_omitted():
    assert canonicalize_v2(
        {"constraints": {"x": None}}
    ) == b'{"constraints":{}}'


def test_f3_mixed_object_omits_null_and_preserves_remaining_members():
    assert canonicalize_v2(
        {
            "constraints": {
                "a": 1,
                "b": None,
                "c": 1.5,
            }
        }
    ) == b'{"constraints":{"a":1,"c":1.5}}'


def test_f4_null_array_element_is_preserved():
    assert canonicalize_v2(
        {"constraints": {"x": [1, None, 2]}}
    ) == b'{"constraints":{"x":[1,null,2]}}'


def test_leading_null_valued_object_member_is_omitted():
    assert canonicalize_v2(
        {
            "constraints": {
                "a": None,
                "b": 1,
                "c": 2,
            }
        }
    ) == b'{"constraints":{"b":1,"c":2}}'


def test_trailing_null_valued_object_member_is_omitted():
    assert canonicalize_v2(
        {
            "constraints": {
                "a": 1,
                "b": 2,
                "c": None,
            }
        }
    ) == b'{"constraints":{"a":1,"b":2}}'


def test_nested_null_valued_object_member_is_omitted():
    assert canonicalize_v2(
        {
            "constraints": {
                "outer": {
                    "keep": 1,
                    "omit": None,
                }
            }
        }
    ) == b'{"constraints":{"outer":{"keep":1}}}'


@pytest.mark.parametrize(
    "value",
    [
        math.inf,
        -math.inf,
        math.nan,
    ],
)
def test_non_finite_numbers_are_rejected(value):
    with pytest.raises(
        ValueError,
        match="Non-finite numbers",
    ):
        canonicalize_v2(
            {"constraints": {"x": value}}
        )
