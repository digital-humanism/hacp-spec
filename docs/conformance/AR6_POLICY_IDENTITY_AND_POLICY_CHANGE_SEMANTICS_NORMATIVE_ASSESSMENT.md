# AR-6 — Policy Identity and Policy-Change Semantics
## Normative Assessment

**Status:** Complete
**Production RED:** NO
**Production changes:** NONE
**Primary normative owner:** HACP-Core

---

## 1. Decision question

AR-6 must resolve:

```text
Should every policy transition automatically invalidate
DecisionTokens issued under an earlier policy?
```

The conclusion of this assessment is:

```text
NO — not automatically.
```

A policy transition and termination of already-issued authority are distinct security events.

---

## 2. Established baseline

Current HACP-Core establishes:

```text
policy_digest
=
SHA-256(canonical policy definition
used at decision time)
```

The digest is:

- present in authorization decision state;
- bound into an ALLOW DecisionToken;
- protected by the token signature;
- usable as a traceability anchor for the policy that produced the decision.

Therefore:

```text
policy_digest establishes decision-time policy identity.
```

It does not by itself establish:

```text
currently effective policy identity at execution time.
```

---

## 3. Normative distinction

AR-6 adopts the following distinction:

```text
policy transition
!=
authority invalidation
```

and:

```text
policy identity binding
!=
policy lifecycle invalidation
```

A change from:

```text
P1 → P2
```

does not, solely because the policy definition changed, establish that every authorization previously issued under P1 must become unusable.

---

## 4. Why automatic invalidation is rejected

A universal rule:

```text
P1 → P2
therefore every token(P1) becomes invalid
```

would require substantially more normative machinery than currently exists.

At minimum HACP would have to define:

```text
- authoritative current policy;
- policy activation instant;
- policy generation/version ordering;
- policy distribution;
- convergence across enforcement points;
- maximum policy staleness;
- transition atomicity;
- restart semantics;
- rollback semantics;
- failure behavior;
- treatment of in-flight requests.
```

None of these semantics are presently defined sufficiently to support a deterministic execution-time policy equality rule.

Automatic invalidation would therefore introduce a new distributed authorization protocol rather than clarify the existing token model.

---

## 5. Not every policy transition changes existing authority

A policy definition may change for reasons that do not require cancellation of every previously issued authorization.

Examples include changes that are:

```text
- unrelated to the action represented by an existing token;
- applicable only to new evaluations;
- administrative or descriptive;
- relevant to another principal/resource class;
- stricter only for future authorization issuance.
```

Therefore a global rule:

```text
any policy digest change
→ invalidate all earlier tokens
```

would couple policy deployment to authorization cancellation more strongly than necessary.

---

## 6. Explicit invalidation model

AR-6 instead separates:

```text
policy evaluation
```

from:

```text
authority lifecycle
```

Model:

```text
Policy P1
→ evaluate()
→ ALLOW
→ DecisionToken(policy_digest=P1)
```

Later:

```text
Policy P2 becomes applicable to new evaluations
```

does not alone decide the lifecycle of the existing token.

If previously issued authority is to be terminated as part of a policy transition, the existing revocation, expiry, or other normatively defined invalidation mechanisms provide the means to terminate that authority.

Existing mechanisms include:

```text
- token revocation;
- envelope revocation;
- signing-key revocation;
- token expiry;
- other already-defined enforcement constraints.
```

Therefore:

```text
policy transition requiring withdrawal of existing authority
→ explicit invalidation
```

rather than:

```text
policy transition
→ implicit universal invalidation
```

---

## 7. Security consequence

This model preserves two independently testable properties.

### Property A — Policy identity integrity

```text
An issued token cannot silently claim that its decision
was made under a different policy.
```

This is provided by:

```text
policy_digest
+
token signature
```

### Property B — Authority termination

```text
Previously issued authority can be terminated
when policy/security state requires it.
```

This is provided by explicit lifecycle controls such as:

```text
revocation
expiry
```

The two properties must not be conflated.

---

## 8. Threat-model correspondence

The existing threat-model statement:

```text
Policy digest binding ensures tokens cannot be reused
if policy changes.
```

over-compresses these two properties.

`policy_digest` binding by itself establishes policy identity integrity.

It does not independently establish that:

```text
policy change
→ old token unusable
```

A more precise security model is:

```text
Policy digest binding prevents undetected substitution
of the policy identity associated with an issued token.

Policy transitions that require previously issued authority
to be withdrawn rely on the existing revocation, expiry,
or other normatively defined invalidation mechanisms
for that withdrawal.
```

---

## 9. Key-compromise context

This distinction is especially important because the current threat-model sentence appears in the `Key Compromise` section.

If an attacker possesses a valid signing key, `policy_digest` does not by itself prevent that attacker from issuing a new correctly signed token containing another policy digest.

Primary mitigation for key compromise therefore remains:

```text
signing-key revocation
+
short token lifetime
```

Policy identity binding is complementary integrity metadata; it is not an independent cure for compromised signing authority.

---

## 10. Enforcement implications

Under the AR-6 model, Enforcement is not required merely by the existence of `policy_digest` to maintain:

```text
currentPolicyDigest
```

or to perform:

```text
token.PolicyDigest == currentPolicyDigest
```

on every request.

Such a requirement would require a separately defined normative mechanism.

Existing production sidecar behavior therefore does not currently violate AR-6.

---

## 11. Meaning of policy freshness

AR-6 does not reopen AR-4.

The Enforcement requirement concerning:

```text
revocation and policy freshness
```

must not be interpreted, without additional normative text, as an implicit requirement for:

```text
current-policy digest equality
```

Policy freshness may later receive a more precise lifecycle definition, but AR-6 does not invent that mechanism.

---

## 12. Final lifecycle model

AR-6 adopts the following conceptual model:

```text
                 decision time

Policy P1
   |
   v
evaluate()
   |
   v
ALLOW
   |
   v
DecisionToken
policy_digest = digest(P1)
   |
   +----------------------+
   |                      |
   v                      v
traceability          signed identity
```

Later:

```text
Policy P2
```

does not automatically rewrite the meaning of the already-issued token.

Instead:

```text
P1 → P2
```

and:

```text
withdraw token(P1)
```

are separate operations unless a future normative profile explicitly defines them as coupled.

---

## 13. Normative conclusion

AR-6 establishes:

```text
policy_digest is a decision-time policy identity binding.
```

AR-6 rejects the inference:

```text
policy_digest exists
→ Enforcement must know current policy
→ every policy change invalidates every older token.
```

Instead:

```text
policy transition
and
withdrawal of existing authorization
are separate normative concepts.
```

```text
If previously issued authority is to be terminated as part of
a policy transition, the existing revocation, expiry, or other
normatively defined invalidation mechanisms provide the means
to terminate that authority.
```

---

## 14. Production assessment

```text
Production current-policy state:
NOT REQUIRED by this AR-6 model

Production PolicyDigest comparator:
NOT REQUIRED

Existing sidecar violation:
NO

Production RED:
NO

Production fix:
NOT REQUIRED

Production changes:
0
```

---

## 15. Documentation consequence

The required documentation correspondence correction was limited to:

```text
threat-model.md
```

The correction clarifies that cryptographic `policy_digest` binding preserves the identity of the policy under which a token was issued, and does not, by itself, establish automatic invalidation of that token following a later policy transition.

Where a policy transition requires previously issued authority to be withdrawn, the existing revocation, expiry, or other normatively defined invalidation mechanisms provide the means for that withdrawal.

No production implementation, profile, vector, or test changes were required.

The documentation correction was completed in:

```text
d0eddba
docs: clarify policy digest transition semantics
```

---

## 16. Explicitly rejected changes

AR-6 does not authorize:

```text
- adding currentPolicyDigest to sidecar;
- adding a policy store;
- adding policy streaming;
- adding policy revision messages;
- adding new control-plane state;
- adding token/current-policy comparison;
- adding vectors for such comparison;
- adding tests for semantics not already normative.
```

---

## 17. Final decision

```text
AR-6 NORMATIVE DESIGN DECISION

Decision-time policy identity:
REQUIRED

Cryptographic policy identity binding:
REQUIRED

Automatic invalidation on every policy transition:
NOT REQUIRED

Explicit authority withdrawal mechanisms:
AVAILABLE through existing normative lifecycle controls

Production RED:
NO

Production changes:
NONE

Documentation correction:
COMPLETED — threat-model wording clarified
```

Central invariant:

```text
policy change
!=
authority revocation
```

and:

```text
signed policy identity
!=
automatic policy-transition invalidation
```
