import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
BAKER = REPO_ROOT / "tools" / "enforcement_v2_bake_vectors.py"


class CliNegativeVectorBakingTests(unittest.TestCase):
    def test_cli_bakes_negative_vector_instead_of_skipping_it(self):
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

        with tempfile.TemporaryDirectory() as tmp:
            vectors_dir = Path(tmp)
            vector_path = vectors_dir / "hc2_002_negative.json"

            vector_path.write_text(
                json.dumps(vector, indent=2) + "\n",
                encoding="utf-8",
            )

            result = subprocess.run(
                [
                    sys.executable,
                    str(BAKER),
                    "--vectors-dir",
                    str(vectors_dir),
                ],
                cwd=REPO_ROOT,
                capture_output=True,
                text=True,
            )

            self.assertEqual(
                result.returncode,
                0,
                msg=result.stdout + result.stderr,
            )

            baked = json.loads(
                vector_path.read_text(encoding="utf-8")
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


if __name__ == "__main__":
    unittest.main()