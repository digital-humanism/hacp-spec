import copy
import io
import sys
import unittest
from contextlib import redirect_stdout
from pathlib import Path

TOOLS_DIR = Path(__file__).resolve().parents[1]
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import enforcement_v2_bake_vectors


class NegativeVectorBakingTests(unittest.TestCase):
    def test_negative_enforcement_vector_is_cryptographically_baked(self):
        vector = {
            "test_id": "ENF-HC2-002",
            "type": "negative",
            "inputs": {
                "intent_envelope": {
                    "signer_key_id": "key-ed25519-test-001",
                    "signature": "PLACEHOLDER",
                },
                "proposed_action": {
                    "hacp_version": "0.9",
                    "action_id": "11111111-1111-1111-1111-111111111111",
                },
                "decision_token": {
                    "action_hash": "PLACEHOLDER",
                    "signer_key_id": "key-ed25519-test-001",
                    "signature": "PLACEHOLDER",
                    "constraints": {
                        "path": "/a/b",
                    },
                },
                "http_request": {
                    "method": "GET",
                    "request_target": "/a%2Fb",
                },
            },
            "policy_context": {
                "clock": 1786000300,
            },
            "expected": {
                "outcome": "DENY",
                "reason_codes": ["SCOPE_EXCEEDED"],
            },
            "draft_mode": True,
        }

        original = copy.deepcopy(vector)

        baked = enforcement_v2_bake_vectors.bake_vector(
            vector,
            enforcement_v2_bake_vectors.load_signing_key(),
        )

        self.assertNotEqual(
            baked["inputs"]["decision_token"]["action_hash"],
            "PLACEHOLDER",
        )
        self.assertNotEqual(
            baked["inputs"]["intent_envelope"]["signature"],
            "PLACEHOLDER",
        )
        self.assertNotEqual(
            baked["inputs"]["decision_token"]["signature"],
            "PLACEHOLDER",
        )
        self.assertFalse(baked["draft_mode"])

        self.assertEqual(
            baked["inputs"]["decision_token"]["constraints"],
            original["inputs"]["decision_token"]["constraints"],
        )
        self.assertEqual(
            baked["inputs"]["http_request"],
            original["inputs"]["http_request"],
        )

    def test_tampered_baked_negative_vector_fails_verification(self):
        vector = {
            "test_id": "ENF-HC2-002",
            "type": "negative",
            "inputs": {
                "intent_envelope": {
                    "signer_key_id": "key-ed25519-test-001",
                    "signature": "PLACEHOLDER",
                },
                "proposed_action": {
                    "hacp_version": "0.9",
                    "action_id": "11111111-1111-1111-1111-111111111111",
                },
                "decision_token": {
                    "action_hash": "PLACEHOLDER",
                    "signer_key_id": "key-ed25519-test-001",
                    "signature": "PLACEHOLDER",
                    "constraints": {
                        "path": "/a/b",
                    },
                },
                "http_request": {
                    "method": "GET",
                    "request_target": "/a%2Fb",
                },
            },
            "policy_context": {
                "clock": 1786000300,
            },
            "expected": {
                "outcome": "DENY",
                "reason_codes": ["SCOPE_EXCEEDED"],
            },
            "draft_mode": True,
        }

        private_key = enforcement_v2_bake_vectors.load_signing_key()
        baked = enforcement_v2_bake_vectors.bake_vector(
            vector,
            private_key,
        )

        self.assertTrue(
            enforcement_v2_bake_vectors.verify_vector(
                baked,
                private_key,
            )
        )

        baked["inputs"]["decision_token"]["action_hash"] = "0" * 64

        output = io.StringIO()

        with redirect_stdout(output):
            verified = enforcement_v2_bake_vectors.verify_vector(
                baked,
                private_key,
            )

        self.assertFalse(verified)
        self.assertIn("action_hash mismatch", output.getvalue())


if __name__ == "__main__":
    unittest.main()