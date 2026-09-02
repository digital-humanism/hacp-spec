# HACP Non-Goals

**Version:** 1.0.0
**Status:** Stable
**License:** CC BY 4.0  

This document enumerates what the Human Agency Continuity Protocol (HACP) deliberately does not attempt. Its purpose is to keep the standard focused, strictly testable, and implementable without reliance on any specific SDK or external infrastructure.

---

## 1. Post-hoc Content Marking
HACP does not watermark AI output and does not replace C2PA or similar content-provenance systems. HACP authorizes actions **before execution**. Detecting, labeling, or tracing "AI slop" after generation is explicitly out of scope.

## 2. Prompt-Injection Firewall
HACP assumes proposed actions arrive through a mediated, schema-constrained interface (e.g., structured tool calls). It does not analyze raw prompts, user inputs, or model internals. Deployments permitting unmediated free-form tool execution or uncontrolled egress are out of scope for HACP-Core compliance and require HACP-Enforcement or equivalent external controls.

## 3. Identity Mesh / SSO
The `principal` field is an opaque string identifier. HACP does not prescribe, build, or integrate with any authentication infrastructure, identity providers, or Single Sign-On (SSO) systems. Minimal signer assurance for human approvals is defined in HACP-Runtime; a global identity system is not.

## 4. DLP / Content Filtering
HACP controls action boundaries and rights (`verb`, `audience`, `externality`, `reversibility`, `data_class`, `destination`, `quantity`). It does not inspect, parse, or filter the semantic content of data payloads.

## 5. OS-Level Sandboxing and Kernel Enforcement
HACP-Enforcement covers L7 mediation (e.g., MCP tool path + explicit HTTP proxy). eBPF, iptables, seccomp, container isolation, and other kernel-level controls belong to external infrastructure and are out of scope.

## 6. Service Mesh / Traffic Management
HACP is not a networking or traffic-management platform. It carries authorization semantics, not routing, load balancing, rate limiting, or observability primitives.

## 7. Global Revocation Consensus
Revocation propagation is deployment-scoped (e.g., signed push + local denylist with bounded staleness). HACP does not define or require a distributed consensus protocol (e.g., Raft, Paxos) for global state synchronization.

## 8. Model Alignment and Training Safety
HACP constrains action execution at runtime. It does not govern model training, fine-tuning, alignment, or output quality. It is orthogonal to model-level safety research.

## 9. Regulatory Certification
HACP provides engineering controls, deterministic boundaries, and auditability. It does not itself certify compliance with any regulation (e.g., EU AI Act, GDPR). It supplies verifiable evidence that may support such compliance audits.

## 10. The Boundary Rule
Any capability, feature, or behavior not explicitly required by a normative invariant or a normative section of an active profile document is out of scope until formally added through the versioning policy (`versioning.md`). Implementations MUST NOT invent normative requirements beyond this specification.
