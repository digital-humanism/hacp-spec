import io
import json
import sys
import unittest
from pathlib import Path


HARNESS_DIR = Path(__file__).resolve().parents[1]
if str(HARNESS_DIR) not in sys.path:
    sys.path.insert(0, str(HARNESS_DIR))

import enforcement_v2_runner


class FakeProcess:
    def __init__(self):
        self.stdin = io.BytesIO()
        self.stdout = io.BytesIO(
            b'{"protocol_version":"1","decision":"DENY","reason_codes":[]}\n'
        )
        self.stderr = io.BytesIO()

    def poll(self):
        return None


class RunnerInputPassthroughTests(unittest.TestCase):
    def test_profile_specific_http_request_input_is_preserved(self):
        target = enforcement_v2_runner.RunnerTarget(
            runner_command=["unused"]
        )
        target.process = FakeProcess()

        vector = {
            "test_id": "HC2-INPUT-001",
            "inputs": {
                "intent_envelope": {},
                "proposed_action": {},
                "decision_token": None,
                "http_request": {
                    "method": "GET",
                    "request_target": "/a%2Fb"
                }
            },
            "policy_context": {}
        }

        target.evaluate(vector)

        encoded_request = target.process.stdin.getvalue().decode("utf-8")
        request = json.loads(encoded_request)

        self.assertIn("http_request", request["input"])
        self.assertEqual(
            request["input"]["http_request"],
            {
                "method": "GET",
                "request_target": "/a%2Fb"
            }
        )


if __name__ == "__main__":
    unittest.main()
