#!/usr/bin/env python3
"""
HACP Conformance Harness v0.9.2

Cross-language conformance testing harness for HACP implementations.
Follows industry-standard architecture (OAuth 2.0, OpenAPI, C2PA pattern).

Architecture:
- Vectors: Language-independent JSON test cases
- Generator: Creates valid signatures and hashes for golden vectors
- Verifier: Validates responses from target implementations
- Target Interface: HTTP server or CLI processor

Modes:
- local:    Emulates HACP-Core logic (for spec validation)
- http:     Tests HTTP server implementation
- cli:      Tests CLI implementation
"""

import argparse
import json
import hashlib
import base64
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Dict, Any, Optional, List
from dataclasses import dataclass
from enum import Enum

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False

try:
    from cryptography.hazmat.primitives.asymmetric.ed25519 import (
        Ed25519PrivateKey,
        Ed25519PublicKey
    )
    HAS_CRYPTO = True
except ImportError:
    HAS_CRYPTO = False
    Ed25519PrivateKey = None
    Ed25519PublicKey = None

try:
    import jsonschema
    HAS_JSONSCHEMA = True
except ImportError:
    HAS_JSONSCHEMA = False


class TargetType(Enum):
    LOCAL = "local"
    HTTP = "http"
    CLI = "cli"


@dataclass
class TestResult:
    test_id: str
    test_type: str
    description: str
    passed: bool
    details: Dict[str, Any]


# --- Configuration ---
SPEC_ROOT = Path(__file__).parent.parent
SCHEMAS_DIR = SPEC_ROOT / "schemas"
VECTORS_DIR = SPEC_ROOT / "vectors"


# --- Canonicalization ---
def canonicalize(obj: Any) -> bytes:
    """
    Strict JCS-like canonicalization for deterministic serialization.
    Used for hashing and signing operations.
    """
    if obj is None:
        return b"null"
    if isinstance(obj, bool):
        return b"true" if obj else b"false"
    if isinstance(obj, int):
        return str(obj).encode("utf-8")
    if isinstance(obj, str):
        return json.dumps(obj, ensure_ascii=False).encode("utf-8")
    if isinstance(obj, list):
        return b"[" + b",".join(canonicalize(item) for item in obj) + b"]"
    if isinstance(obj, dict):
        sorted_keys = sorted(obj.keys())
        pairs = [canonicalize(k) + b":" + canonicalize(obj[k]) for k in sorted_keys]
        return b"{" + b",".join(pairs) + b"}"
    raise ValueError(f"Unsupported type: {type(obj)}")


def compute_sha256(data: bytes) -> str:
    """Compute SHA-256 hash as lowercase hex string."""
    return hashlib.sha256(data).hexdigest()


# --- Crypto ---
def sign_payload(payload_bytes: bytes, private_key) -> str:
    """Sign payload and return Base64url encoded signature."""
    signature = private_key.sign(payload_bytes)
    return base64.urlsafe_b64encode(signature).rstrip(b"=").decode("utf-8")


def verify_signature(payload_bytes: bytes, signature_b64: str, public_key) -> bool:
    """Verify Ed25519 signature."""
    try:
        sig_bytes = base64.urlsafe_b64decode(
            signature_b64 + "=" * (-len(signature_b64) % 4)
        )
        public_key.verify(sig_bytes, payload_bytes)
        return True
    except Exception:
        return False


# --- Schema Loading ---
def load_schemas() -> Dict[str, Any]:
    """Load all JSON schemas from schemas/ directory."""
    schemas = {}
    if not SCHEMAS_DIR.exists():
        print(f"Warning: schemas directory not found at {SCHEMAS_DIR}")
        return schemas

    for schema_file in SCHEMAS_DIR.glob("*.json"):
        try:
            with open(schema_file, "r", encoding="utf-8") as f:
                schema = json.load(f)
            schemas[schema.get("title", schema_file.stem)] = schema
        except Exception as e:
            print(f"Warning: failed to load schema {schema_file.name}: {e}")

    return schemas


# --- Generator ---
class KeyLoader:
    """
    Loads fixed test keypair for signature verification.
    Verifier-only: no key generation, no signing.
    """

    def __init__(self):
        if not HAS_CRYPTO:
            raise ImportError("cryptography library required")
        
        self.keys_dir = SPEC_ROOT / "harness" / "keys"
        self.public_key = self._load_public_key()
        self.signer_key_id = "key-ed25519-test-001"

    def _load_public_key(self):
        """Load Ed25519 public key from fixed test key file."""
        pub_file = self.keys_dir / "test-ed25519-001.pub"
        if not pub_file.exists():
            raise FileNotFoundError(
                f"Public key not found at {pub_file}. "
                "Run 'python tools/gen_test_keys.py' first."
            )
        
        pub_hex = pub_file.read_text(encoding="utf-8").strip()
        pub_bytes = bytes.fromhex(pub_hex)
        
        if len(pub_bytes) != 32:
            raise ValueError(f"Public key must be 32 bytes, got {len(pub_bytes)}")
        
        return Ed25519PublicKey.from_public_bytes(pub_bytes)


# --- Verifier ---
class ResponseVerifier:
    """
    Verifies responses from target implementations.
    Checks decision outcome, token binding, and cryptographic signatures.
    """

    def __init__(self, public_key=None):
        self.public_key = public_key

    def verify_response(self, vector: Dict, response: Dict) -> TestResult:
        """
        Verify response from target implementation.

        Args:
            vector: Original test vector
            response: Response from target (AgencyDecision format)

        Returns:
            TestResult with pass/fail and details
        """
        test_id = vector["test_id"]
        test_type = vector["type"]
        description = vector["description"]
        expected_outcome = vector["expected"]["outcome"]

        details = {}
        passed = True

        # Check decision outcome
        actual_outcome = response.get("decision")
        if actual_outcome != expected_outcome:
            passed = False
            details["outcome_mismatch"] = {
                "expected": expected_outcome,
                "actual": actual_outcome
            }
        else:
            details["outcome_correct"] = True

        # If ALLOW, verify token binding
        if actual_outcome == "ALLOW" and "decision_token" in response:
            token = response["decision_token"]
            action = vector["inputs"]["proposed_action"]

            # Verify action_hash binding
            canonical_action = canonicalize(action)
            computed_hash = compute_sha256(canonical_action)
            token_hash = token.get("action_hash")

            if token_hash != computed_hash:
                passed = False
                details["action_hash_mismatch"] = {
                    "expected": computed_hash,
                    "actual": token_hash
                }
            else:
                details["action_hash_correct"] = True

            # Verify signature if public key available
            if self.public_key and "signature" in token:
                token_for_verify = {
                    k: v for k, v in token.items() if k != "signature"
                }
                sig_valid = verify_signature(
                    canonicalize(token_for_verify),
                    token["signature"],
                    self.public_key
                )
                if not sig_valid:
                    passed = False
                    details["signature_invalid"] = True
                else:
                    details["signature_valid"] = True

        # For negative vectors, ensure they failed appropriately
        if test_type == "negative" and actual_outcome == "ALLOW":
            passed = False
            details["negative_should_fail"] = True

        return TestResult(
            test_id=test_id,
            test_type=test_type,
            description=description,
            passed=passed,
            details=details
        )


# --- Enhanced Policy Logic ---
def evaluate_logic(action: Dict, envelope: Dict, context: Dict, 
                   token: Optional[Dict] = None) -> str:
    """
    Simulates the HACP-Core evaluate() function based on policy context.
    Returns: 'ALLOW', 'DENY', or 'CHECKPOINT'
    """

    # Check envelope expiry
    current_time = context.get("current_time", envelope.get("issued_at", 0))
    envelope_expires = envelope.get("expires_at")
    if envelope_expires and current_time > envelope_expires:
        return "DENY"

    # Check envelope revocation
    revoked_envelopes = context.get("revoked_envelopes", [])
    if envelope.get("envelope_id") in revoked_envelopes:
        return "DENY"

    # Check token revocation from inputs or context
    if token:
        revoked_tokens = context.get("revoked_tokens", [])
        if token.get("token_id") in revoked_tokens:
            return "DENY"  # TOKEN_REVOKED

        # Check token expiry
        token_expires = token.get("expires_at")
        if token_expires and current_time > token_expires:
            return "DENY"  # TOKEN_EXPIRED

    # INV-5: Validate signer_key_id against trusted keys
    trusted_keys = context.get("trusted_keys")
    if trusted_keys:
        envelope_key = envelope.get("signer_key_id", "")
        if envelope_key not in trusted_keys:
            return "DENY"  # SIGNATURE_FAILURE

        if token:
            token_key = token.get("signer_key_id", "")
            if token_key not in trusted_keys:
                return "DENY"  # SIGNATURE_FAILURE

    # INV-5: Reject non-Ed25519 algorithms
    envelope_key = envelope.get("signer_key_id", "")
    if "hmac" in envelope_key.lower():
        return "DENY"  # SIGNATURE_FAILURE - algorithm not permitted

    if token:
        token_key = token.get("signer_key_id", "")
        if "hmac" in token_key.lower():
            return "DENY"  # SIGNATURE_FAILURE - algorithm not permitted

    # INV-7: Bounded Autonomy
    budget = envelope.get("autonomy_budget", {})
    max_actions = budget.get("max_actions")
    current_count = context.get("current_action_count", 0)
    if max_actions is not None and current_count >= max_actions:
        return "DENY"

    # INV-1: Human Final Decision
    human_verbs = context.get("human_required_verbs", [])
    if action.get("verb") in human_verbs:
        if envelope.get("principal_kind") == "system":
            parent_envelope_id = envelope.get("parent_envelope_id")
            if not parent_envelope_id:
                return "CHECKPOINT"

    # INV-2: Boundary Re-Authorization (Scope Check)
    scope = envelope.get("scope", {})

    allowed_audiences = scope.get("audiences", [])
    if action.get("audience") not in allowed_audiences:
        return "DENY"

    allowed_reversibility = scope.get("reversibility", [])
    if action.get("reversibility") not in allowed_reversibility:
        return "DENY"

    allowed_externality = scope.get("externality", [])
    if action.get("externality") not in allowed_externality:
        return "DENY"

    allowed_data_classes = scope.get("data_classes", [])
    if action.get("data_class") not in allowed_data_classes:
        return "DENY"

    allowed_verbs = scope.get("verbs", [])
    if action.get("verb") not in allowed_verbs:
        return "DENY"

    allowed_resources = scope.get("resource_classes", [])
    if action.get("resource_class") not in allowed_resources:
        return "DENY"

    if "quantity" in action:
        max_quantity = scope.get("max_quantity")
        if max_quantity is not None and action["quantity"] > max_quantity:
            return "DENY"

    if "destination" in action:
        allowed_destinations = scope.get("destinations", [])
        if allowed_destinations and action["destination"] not in allowed_destinations:
            return "DENY"

    if "tool_name" in action:
        allowed_tools = scope.get("tool_names", [])
        if allowed_tools and action["tool_name"] not in allowed_tools:
            return "DENY"

    return "ALLOW"


# --- Target Interfaces ---
class LocalTarget:
    """
    Local emulation target for spec validation.
    Simulates HACP-Core logic without external dependencies.
    """

    def __init__(self, key_loader: KeyLoader):
        self.key_loader = key_loader

    def evaluate(self, vector: Dict) -> Dict:
        """Evaluate vector using local emulation."""
        # Vectors are already baked — no signing in runtime
        action = vector["inputs"]["proposed_action"]
        envelope = vector["inputs"]["intent_envelope"]
        context = vector.get("policy_context", {})
        token = vector["inputs"].get("decision_token")

        # Step 1: Policy evaluation
        decision = evaluate_logic(action, envelope, context, token)

        # Step 2: Crypto verification (INV-3, INV-5)
        if decision == "ALLOW" and token:
            # INV-3: Token Binding - verify action_hash
            canonical_action = canonicalize(action)
            computed_hash = compute_sha256(canonical_action)

            if token.get("action_hash") != computed_hash:
                decision = "DENY"
            else:
                # INV-5: Signature verification
                token_for_verify = {
                    k: v for k, v in token.items() if k != "signature"
                }
                sig_valid = verify_signature(
                    canonicalize(token_for_verify),
                    token["signature"],
                    self.key_loader.public_key
                )
                if not sig_valid:
                    decision = "DENY"

        response = {"decision": decision}

        if decision == "ALLOW" and token:
            response["decision_token"] = token

        return response


class HTTPTarget:
    """
    HTTP server target for clean-room implementations.
    Sends complete vectors to POST /evaluate endpoint.
    """

    def __init__(
        self,
        base_url: str,
        api_key: Optional[str] = None,
        timeout: int = 30
    ):
        if not HAS_REQUESTS:
            raise ImportError("requests library required for HTTP target")

        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self.session = requests.Session()

        self.session.headers.update({
            "Content-Type": "application/json",
            "Accept": "application/json"
        })

        if api_key:
            self.session.headers["Authorization"] = f"Bearer {api_key}"

    def evaluate(self, vector: Dict) -> Dict:
        """Send vector to HTTP endpoint and get response."""
        url = f"{self.base_url}/evaluate"
        response = self.session.post(url, json=vector, timeout=self.timeout)
        response.raise_for_status()
        return response.json()


class CLITarget:
    """
    CLI processor target for clean-room implementations.
    Executes binary with vector path and parses stdout JSON.
    """

    def __init__(self, binary_path: str, timeout: int = 30):
        self.binary_path = binary_path
        self.timeout = timeout

    def evaluate(self, vector: Dict) -> Dict:
        """Execute CLI binary with vector and parse response."""
        # Write vector to temporary file
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".json", delete=False
        ) as f:
            json.dump(vector, f)
            temp_path = f.name

        try:
            # Execute binary
            result = subprocess.run(
                [self.binary_path, "evaluate", "--vector", temp_path],
                capture_output=True,
                text=True,
                timeout=self.timeout
            )

            if result.returncode != 0:
                raise RuntimeError(
                    f"CLI failed with code {result.returncode}: {result.stderr}"
                )

            # Parse stdout JSON
            return json.loads(result.stdout)

        finally:
            # Clean up temp file
            Path(temp_path).unlink()


# --- Main Harness ---
class HACPHarness:
    """
    Main conformance testing harness.
    Orchestrates vector loading, target execution, and response verification.
    """

    def __init__(self, target, verifier: ResponseVerifier, schemas: Dict):
        self.target = target
        self.verifier = verifier
        self.schemas = schemas

    def run_test(self, vector: Dict) -> TestResult:
        """Run single test vector against target."""
        try:
            # Validate vector schema (pre-flight)
            if HAS_JSONSCHEMA and self.schemas:
                self._validate_vector_schemas(vector)

            # Execute against target
            response = self.target.evaluate(vector)

            # Verify response
            return self.verifier.verify_response(vector, response)

        except Exception as e:
            return TestResult(
                test_id=vector["test_id"],
                test_type=vector["type"],
                description=vector["description"],
                passed=False,
                details={"error": str(e)}
            )

    def run_all(self, vectors_dir: Path) -> Dict[str, Any]:
        """Run all test vectors and return summary."""
        results = []

        for vector_file in sorted(vectors_dir.glob("*.json")):
            try:
                with open(vector_file, "r", encoding="utf-8") as f:
                    vector = json.load(f)

                result = self.run_test(vector)
                results.append(result)

                # Print result
                status = "PASS" if result.passed else "FAIL"
                print(f"[{status}] {result.test_id}: {result.description}")

                if not result.passed:
                    print(f"       Details: {result.details}")

            except Exception as e:
                print(f"[ERROR] {vector_file.name}: {e}")
                results.append(TestResult(
                    test_id=vector_file.stem,
                    test_type="unknown",
                    description=str(e),
                    passed=False,
                    details={"error": str(e)}
                ))

        # Generate summary
        passed = sum(1 for r in results if r.passed)
        failed = sum(1 for r in results if not r.passed)

        return {
            "total": len(results),
            "passed": passed,
            "failed": failed,
            "results": results
        }

    def _validate_vector_schemas(self, vector: Dict):
        """Validate vector against JSON schemas."""
        # Basic structure validation
        assert "test_id" in vector, "Missing test_id"
        assert "type" in vector, "Missing type"
        assert "inputs" in vector, "Missing inputs"
        assert "expected" in vector, "Missing expected"


# --- CLI ---
def main():
    parser = argparse.ArgumentParser(
        description="HACP Conformance Harness - Cross-language testing tool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Local mode (spec validation):
  python harness.py --mode local

  # HTTP target (clean-room server):
  python harness.py --mode http --target-url http://localhost:8080

  # CLI target (clean-room binary):
  python harness.py --mode cli --binary-path ./hacp-go
        """
    )

    parser.add_argument(
        "--mode",
        choices=["local", "http", "cli"],
        default="local",
        help="Target mode: local (emulation), http (server), or cli (binary)"
    )
    parser.add_argument(
        "--target-url",
        default=None,
        help="Base URL for HTTP mode"
    )
    parser.add_argument(
        "--api-key",
        default=None,
        help="API key for HTTP mode"
    )
    parser.add_argument(
        "--binary-path",
        help="Path to CLI binary for CLI mode"
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=30,
        help="Timeout in seconds (default: 30)"
    )
    parser.add_argument(
        "--vectors-dir",
        type=Path,
        default=VECTORS_DIR,
        help="Path to vectors directory"
    )
    parser.add_argument(
        "--output",
        choices=["console", "json"],
        default="console",
        help="Output format: console (human-readable) or json"
    )

    args = parser.parse_args()

    # Validate mode-specific arguments
    if args.mode == "http":
        if not args.target_url:
            print("Error: --target-url required for HTTP mode")
            sys.exit(1)
        if not HAS_REQUESTS:
            print("Error: requests library required for HTTP mode")
            sys.exit(1)

    if args.mode == "cli":
        if not args.binary_path:
            print("Error: --binary-path required for CLI mode")
            sys.exit(1)
        if not Path(args.binary_path).exists():
            print(f"Error: binary not found at {args.binary_path}")
            sys.exit(1)

    # Load schemas
    schemas = load_schemas()

    # Create key loader (for local mode and verification)
    key_loader = KeyLoader() if HAS_CRYPTO else None

    # Create target
    if args.mode == "local":
        if not key_loader:
            print("Error: cryptography library required for local mode")
            sys.exit(1)
        target = LocalTarget(key_loader)
    elif args.mode == "http":
        target = HTTPTarget(
            base_url=args.target_url,
            api_key=args.api_key,
            timeout=args.timeout
        )
    elif args.mode == "cli":
        target = CLITarget(
            binary_path=args.binary_path,
            timeout=args.timeout
        )
    else:
        print(f"Unknown mode: {args.mode}")
        sys.exit(1)

    # Create verifier
    verifier = ResponseVerifier(
        public_key=key_loader.public_key if key_loader else None
    )

    # Create harness
    harness = HACPHarness(target, verifier, schemas)

    # Run tests
    print("=" * 60)
    print(f"HACP Conformance Harness v0.9.2 - Mode: {args.mode}")
    print("=" * 60)
    print()

    summary = harness.run_all(args.vectors_dir)

    # Print summary
    print()
    print("=" * 60)
    print(f"RESULTS: {summary['passed']}/{summary['total']} passed")
    print("=" * 60)

    if args.output == "json":
        # Output detailed JSON results
        print(json.dumps({
            "summary": {
                "total": summary["total"],
                "passed": summary["passed"],
                "failed": summary["failed"]
            },
            "results": [
                {
                    "test_id": r.test_id,
                    "test_type": r.test_type,
                    "description": r.description,
                    "passed": r.passed,
                    "details": r.details
                }
                for r in summary["results"]
            ]
        }, indent=2))

    # Exit with appropriate code
    sys.exit(0 if summary["failed"] == 0 else 1)


if __name__ == "__main__":
    main()