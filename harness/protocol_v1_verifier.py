"""Protocol v1 response verification."""

from typing import Dict

import harness as base_harness


class ProtocolV1ResponseVerifier(base_harness.ResponseVerifier):
    """Verifies Protocol v1 normative observable response fields."""

    def verify_response(
        self,
        vector: Dict,
        response: Dict,
    ) -> base_harness.TestResult:
        result = super().verify_response(vector, response)

        expected_reason_codes = vector["expected"].get("reason_codes")
        if expected_reason_codes is None:
            return result

        actual_reason_codes = response.get("reason_codes", [])

        if actual_reason_codes != expected_reason_codes:
            result.passed = False
            result.details["reason_codes_mismatch"] = {
                "expected": expected_reason_codes,
                "actual": actual_reason_codes,
            }
        else:
            result.details["reason_codes_correct"] = True

        return result
