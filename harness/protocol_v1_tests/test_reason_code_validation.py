import sys
import unittest
from pathlib import Path


HARNESS_DIR = Path(__file__).resolve().parents[1]
if str(HARNESS_DIR) not in sys.path:
    sys.path.insert(0, str(HARNESS_DIR))

import protocol_v1_verifier


class ProtocolV1ReasonCodeValidationTests(unittest.TestCase):
    def test_mismatched_reason_code_fails_conformance(self):
        verifier = protocol_v1_verifier.ProtocolV1ResponseVerifier()

        vector = {
            "test_id": "CORE-INV2-008",
            "type": "negative",
            "description": "Absent optional security-relevant attribute",
            "inputs": {
                "intent_envelope": {},
                "proposed_action": {},
            },
            "expected": {
                "outcome": "DENY",
                "reason_codes": ["UNKNOWN_ATTRIBUTE"],
            },
        }

        response = {
            "decision": "DENY",
            "reason_codes": ["POLICY_DENIED"],
        }

        result = verifier.verify_response(vector, response)

        self.assertFalse(
            result.passed,
            "Protocol v1 reason-code mismatch must fail conformance",
        )


if __name__ == "__main__":
    unittest.main()
