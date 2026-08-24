#!/usr/bin/env python3
"""
HACP Conformance Harness v0.9.2 - Runner Protocol Implementation

Extends the base harness with language-neutral runner protocol support.
Follows runner_protocol.md specification.

Runner Protocol:
- stdin: JSON requests (one per line)
- stdout: JSON responses (one per line)
- stderr: diagnostic output only
- Exit codes: 0=conformant, 1=failure, 2=harness error, 3=runner error
"""

import argparse
import json
import hashlib
import subprocess
import sys
import time
from pathlib import Path
from typing import Dict, Any, Optional, List
from dataclasses import dataclass
from datetime import datetime

# Import from existing harness
import harness as base_harness


PROTOCOL_VERSION = "1"


@dataclass
class RunnerRequest:
    protocol_version: str
    operation: str
    vector_id: str
    input: Dict[str, Any]


@dataclass
class RunnerResponse:
    protocol_version: str
    decision: str
    reason_codes: List[str]
    action_hash: Optional[str]
    metrics: Optional[Dict[str, Any]]
    error_message: Optional[str] = None


class ConformanceManifest:
    """Loads and verifies conformance manifest."""

    def __init__(self, manifest_path: Path):
        if not manifest_path.exists():
            raise FileNotFoundError(f"Manifest not found: {manifest_path}")

        with open(manifest_path, "r", encoding="utf-8") as f:
            self.data = json.load(f)

        self.spec_version = self.data["spec_version"]
        self.profile = self.data["profile"]
        self.vector_set = self.data["vector_set"]
        self.canonicalization = self.data["canonicalization"]
        self.digest_algorithm = self.data["digest_algorithm"]
        self.expected_digest = self.data["vector_digest"]
        self.total_vectors = self.data["total_vectors"]

    def verify_vector_digest(self, vectors_dir: Path) -> bool:
        """Verify that vectors match the expected digest."""
        if self.digest_algorithm != "SHA-256":
            raise ValueError(f"Unsupported digest algorithm: {self.digest_algorithm}")

        # Load all vectors, sort by filename for deterministic ordering
        vector_files = sorted(vectors_dir.glob("*.json"))

        # Compute digest over canonicalized vector set
        hasher = hashlib.sha256()
        for vf in vector_files:
            with open(vf, "r", encoding="utf-8") as f:
                # Use strict loading to detect duplicates
                try:
                    content = f.read()
                    # Canonicalize the JSON content
                    obj = json.loads(content, object_pairs_hook=base_harness._reject_duplicates)
                    canonical = base_harness.canonicalize(obj)
                    hasher.update(canonical)
                except base_harness.DuplicateKeyError:
                    # Include duplicate-key vectors in digest computation
                    with open(vf, "r", encoding="utf-8") as f2:
                        loose = json.load(f2)
                        canonical = base_harness.canonicalize(loose)
                        hasher.update(canonical)

        computed_digest = f"sha256:{hasher.hexdigest()}"
        return computed_digest == self.expected_digest


class RunnerTarget:
    """
    Runner protocol target for language-neutral conformance testing.
    Communicates with implementation via stdin/stdout JSON streaming.
    Uses binary mode with explicit UTF-8 encoding to avoid Windows cp1251 issues.
    """

    def __init__(
        self,
        runner_command: List[str],
        timeout_ms: int = 5000,
        verbose: bool = False
    ):
        self.runner_command = runner_command
        self.timeout_ms = timeout_ms
        self.verbose = verbose
        self.process: Optional[subprocess.Popen] = None

    def start(self):
        """Start the runner process in binary mode."""
        self.process = subprocess.Popen(
            self.runner_command,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=None if self.verbose else subprocess.DEVNULL,
            text=False,
        )

    def stop(self):
        """Stop the runner process."""
        if self.process:
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()

    def evaluate(self, vector: Dict) -> Dict:
        """Send vector to runner and get response."""
        if not self.process:
            self.start()

        # Build request
        request = {
            "protocol_version": PROTOCOL_VERSION,
            "operation": "evaluate",
            "vector_id": vector["test_id"],
            "input": {
                "intent_envelope": vector["inputs"]["intent_envelope"],
                "proposed_action": vector["inputs"]["proposed_action"],
                "decision_token": vector["inputs"].get("decision_token"),
                "checkpoint": vector["inputs"].get("checkpoint"),
                "provenance_event": vector["inputs"].get("provenance_event"),
                "policy_context": vector.get("policy_context", {})
            }
        }

        # Encode request as UTF-8 bytes
        request_bytes = (json.dumps(request) + "\n").encode('utf-8')

        # Send request
        try:
            self.process.stdin.write(request_bytes)
            self.process.stdin.flush()
        except Exception as e:
            raise RunnerError(f"Failed to send request: {e}")

        # Read response with timeout
        start_time = time.time()
        while True:
            elapsed_ms = (time.time() - start_time) * 1000
            if elapsed_ms > self.timeout_ms:
                raise RunnerError(f"Runner timeout after {self.timeout_ms}ms")

            if self.process.poll() is not None:
                stderr = ""

                if self.process.stderr is not None:
                    stderr = self.process.stderr.read().decode(
                        "utf-8",
                        errors="replace",
                    )

                raise RunnerError(
                    f"Runner exited unexpectedly: {stderr}"
                )

            # Read one line (blocking)
            line = self.process.stdout.readline()
            if line:
                break

            time.sleep(0.01)  # Small sleep to avoid busy-wait

        # Decode response as UTF-8
        try:
            response_data = json.loads(line.decode('utf-8').strip())
        except (json.JSONDecodeError, UnicodeDecodeError) as e:
            raise RunnerError(f"Malformed JSON response: {e}")

        # Validate protocol version
        if response_data.get("protocol_version") != PROTOCOL_VERSION:
            raise RunnerError(
                f"Protocol version mismatch: expected {PROTOCOL_VERSION}, "
                f"got {response_data.get('protocol_version')}"
            )

        # Convert to base harness response format
        decision = response_data.get("decision")
        if decision == "ERROR":
            error_msg = response_data.get("error_message", "Unknown error")
            raise RunnerError(f"Runner internal error: {error_msg}")

        # Build response compatible with base_harness.ResponseVerifier
        response = {"decision": decision}

        # If ALLOW and decision_token was in request, include it in response
        if decision == "ALLOW" and request["input"]["decision_token"]:
            response["decision_token"] = request["input"]["decision_token"]

        # Include action_hash if present
        if "action_hash" in response_data:
            response["action_hash"] = response_data["action_hash"]

        # Map runner protocol provenance_id to the base harness field name.
        if "provenance_id" in response_data:
            response["provenance_event_id"] = response_data["provenance_id"]

        if "provenance_id" in response_data:
            response["provenance_id"] = response_data["provenance_id"]

        # Include metrics if present (optional, not normative)
        if "metrics" in response_data:
            response["metrics"] = response_data["metrics"]

        return response


class RunnerError(Exception):
    """Raised when runner encounters an error (exit code 3)."""
    pass


class RunnerHarness(base_harness.HACPHarness):
    """
    Extended harness with runner protocol support.
    Adds manifest verification and proper exit codes.
    """

    def __init__(
        self,
        target: RunnerTarget,
        verifier: base_harness.ResponseVerifier,
        schemas: Dict,
        manifest: Optional[ConformanceManifest] = None
    ):
        super().__init__(target, verifier, schemas)
        self.manifest = manifest

    def run_all(self, vectors_dir: Path) -> Dict[str, Any]:
        """Run all tests with manifest verification."""
        # Verify manifest if present
        if self.manifest:
            if not self.manifest.verify_vector_digest(vectors_dir):
                raise ManifestError(
                    f"Vector digest mismatch. Expected: {self.manifest.expected_digest}"
                )
            print(f"Manifest verified: {self.manifest.spec_version} "
                  f"({self.manifest.profile})")
            print(f"Vector set: {self.manifest.vector_set}")
            print(f"Digest: {self.manifest.expected_digest}")
            print()

        # Run tests
        return super().run_all(vectors_dir)


class ManifestError(Exception):
    """Raised when manifest verification fails (exit code 2)."""
    pass


def run_runner_mode(args):
    """Run harness in runner protocol mode."""
    # Load manifest if specified
    manifest = None
    if args.manifest:
        try:
            manifest = ConformanceManifest(Path(args.manifest))
        except Exception as e:
            print(f"Error: Failed to load manifest: {e}")
            sys.exit(2)

    # Load schemas
    schemas = base_harness.load_schemas()

    # Create key loader for verification
    key_loader = base_harness.KeyLoader() if base_harness.HAS_CRYPTO else None

    # Create runner target
    runner_command = args.runner_command.split()
    target = RunnerTarget(
        runner_command=runner_command,
        timeout_ms=args.runner_timeout,
        verbose=args.verbose
    )

    # Create verifier
    verifier = base_harness.ResponseVerifier(
        public_key=key_loader.public_key if key_loader else None
    )

    # Create harness
    harness = RunnerHarness(target, verifier, schemas, manifest)

    # Print header
    print("=" * 60)
    print(f"HACP Conformance Harness v0.9.2 - Runner Mode")
    print(f"Protocol version: {PROTOCOL_VERSION}")
    print(f"Runner: {' '.join(runner_command)}")
    if manifest:
        print(f"Spec: {manifest.spec_version} ({manifest.profile})")
    print("=" * 60)
    print()

    # Run tests
    try:
        target.start()
        summary = harness.run_all(args.vectors_dir)
    except KeyboardInterrupt:
        print("\n\nInterrupted by user")
        sys.exit(130)
    except ManifestError as e:
        print(f"\nManifest verification failed: {e}")
        sys.exit(2)
    except RunnerError as e:
        print(f"\nRunner error: {e}")
        sys.exit(3)
    except Exception as e:
        print(f"\nUnexpected error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(2)
    finally:
        target.stop()

    # Print summary
    print()
    print("=" * 60)
    print(f"RESULTS: {summary['passed']}/{summary['total']} passed")
    print("=" * 60)

    if args.output == "json":
        # Build JSON output per runner_protocol.md
        result_data = {
            "spec_version": manifest.spec_version if manifest else "unknown",
            "profile": manifest.profile if manifest else "unknown",
            "implementation": args.implementation_name or "unknown",
            "implementation_version": args.implementation_version or "unknown",
            "protocol_version": PROTOCOL_VERSION,
            "vector_set_digest": manifest.expected_digest if manifest else None,
            "vectors": {
                "total": summary["total"],
                "passed": summary["passed"],
                "failed": summary["failed"]
            },
            "result": "conformant" if summary["failed"] == 0 else "failure",
            "started_at": datetime.utcnow().isoformat() + "Z",
            "duration_ms": 0,  # Reserved by Runner Protocol v1; timing is not currently reported.
            "failures": [
                {
                    "test_id": r.test_id,
                    "test_type": r.test_type,
                    "description": r.description,
                    "details": r.details
                }
                for r in summary["results"] if not r.passed
            ]
        }
        print(json.dumps(result_data, indent=2))

    # Exit with appropriate code
    if summary["failed"] > 0:
        sys.exit(1)  # Conformance failure
    else:
        sys.exit(0)  # Conformant


def main():
    parser = argparse.ArgumentParser(
        description="HACP Conformance Harness - Runner Protocol Mode",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Run conformance tests against Go implementation:
  python harness_runner.py --runner "go run ./cmd/hacp-conformance-runner"

  # With manifest verification:
  python harness_runner.py \\
    --runner "./hacp-go" \\
    --manifest ./conformance_manifest.json \\
    --implementation-name hacp-go \\
    --implementation-version 0.9.2

  # JSON output for CI:
  python harness_runner.py \\
    --runner "./hacp-sidecar-runner" \\
    --output json > results.json
        """
    )

    parser.add_argument(
        "--runner",
        dest="runner_command",
        required=True,
        help="Runner command (e.g., 'go run ./cmd/runner' or './hacp-go')"
    )
    parser.add_argument(
        "--manifest",
        help="Path to conformance manifest JSON"
    )
    parser.add_argument(
        "--vectors-dir",
        type=Path,
        default=base_harness.VECTORS_DIR,
        help="Path to vectors directory"
    )
    parser.add_argument(
        "--runner-timeout",
        type=int,
        default=5000,
        help="Timeout per request in milliseconds (default: 5000)"
    )
    parser.add_argument(
        "--implementation-name",
        help="Implementation name for JSON output"
    )
    parser.add_argument(
        "--implementation-version",
        help="Implementation version for JSON output"
    )
    parser.add_argument(
        "--output",
        choices=["console", "json"],
        default="console",
        help="Output format: console (human-readable) or json"
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Verbose output (include runner stderr)"
    )

    args = parser.parse_args()

    # Validate runner command
    if not args.runner_command.strip():
        print("Error: --runner command cannot be empty")
        sys.exit(2)

    # Validate vectors directory
    if not args.vectors_dir.exists():
        print(f"Error: Vectors directory not found: {args.vectors_dir}")
        sys.exit(2)

    run_runner_mode(args)


if __name__ == "__main__":
    main()