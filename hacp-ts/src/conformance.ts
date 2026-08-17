import * as fs from "fs";
import type { KeyObject } from "crypto";
import { canonicalBytes } from "./canonical";
import { sha256Hex, verifySignature } from "./crypto";

type Obj = Record<string, any>;
export type Decision = "ALLOW" | "DENY" | "CHECKPOINT";

export interface ConformanceResult {
  decision: Decision;
  reason_codes: string[];
  action_hash: string;
  canonical_action: Buffer;
  canonical_envelope: Buffer;
  canonical_token?: Buffer;
  envelope_signature_valid?: boolean;
  token_signature_valid?: boolean;
  provenance_valid?: boolean;
  provenance_event_id?: string;
}

export interface LoadedVector extends Obj {
  _duplicate_json_keys?: string[];
}

const GENESIS_HASH = "0".repeat(64);

function isObject(value: unknown): value is Obj {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function withoutSignature(value: Obj): Obj {
  const { signature: _signature, ...rest } = value;
  return rest;
}

function asList(value: unknown): unknown[] {
  if (Array.isArray(value)) return value;
  if (value === null || value === undefined) return [];
  return [value];
}

function contains(values: unknown, candidate: unknown): boolean {
  return asList(values).some((value) => value === candidate);
}

function clock(context: Obj, envelope: Obj): number {
  const value = context.current_time ?? context.clock;
  if (typeof value === "number" && Number.isInteger(value)) return value;
  const issuedAt = envelope.issued_at;
  return typeof issuedAt === "number" && Number.isInteger(issuedAt) ? issuedAt : 0;
}

function verifySignatureSafe(
  publicKey: KeyObject,
  canonical: Buffer,
  signature: unknown
): boolean {
  if (typeof signature !== "string" || !signature) return false;
  return verifySignature(publicKey, canonical, signature);
}

function scopeReason(action: Obj, envelope: Obj): string | null {
  const scope = envelope.scope;
  if (!isObject(scope)) return "SCOPE_EXCEEDED";

  const checks: Array<[string, string]> = [
    ["audience", "audiences"],
    ["reversibility", "reversibility"],
    ["externality", "externality"],
    ["data_class", "data_classes"],
    ["verb", "verbs"],
    ["resource_class", "resource_classes"],
  ];

  for (const [actionKey, scopeKey] of checks) {
    const allowed = scope[scopeKey];
    if (allowed !== undefined && allowed !== null) {
      if (!(actionKey in action)) return "UNKNOWN_ATTRIBUTE";
      if (!contains(allowed, action[actionKey])) return "BOUNDARY_CROSSING";
    }
  }

  const quantity = action.quantity;
  const maxQuantity = scope.max_quantity;
  if (
    typeof quantity === "number" &&
    Number.isInteger(quantity) &&
    typeof maxQuantity === "number" &&
    Number.isInteger(maxQuantity) &&
    quantity > maxQuantity
  ) return "SCOPE_EXCEEDED";

  const destinations = asList(scope.destinations);
  if (destinations.length > 0) {
    if (!("destination" in action)) return "UNKNOWN_ATTRIBUTE";
    if (!contains(destinations, action.destination)) return "BOUNDARY_CROSSING";
  }

  const toolNames = asList(scope.tool_names);
  if (toolNames.length > 0) {
    if (!("tool_name" in action)) return "UNKNOWN_ATTRIBUTE";
    if (!contains(toolNames, action.tool_name)) return "BOUNDARY_CROSSING";
  }

  return null;
}

function verifyProvenance(event: Obj, prior: Obj | null, publicKey: KeyObject): boolean {
  if (!("payload" in event)) return false;

  let payloadBytes: Buffer;
  try {
    payloadBytes = canonicalBytes(event.payload);
  } catch {
    return false;
  }
  if (sha256Hex(payloadBytes) !== event.payload_hash) return false;

  let expectedPrev = GENESIS_HASH;
  if (prior !== null) {
    try {
      expectedPrev = sha256Hex(canonicalBytes(prior));
    } catch {
      return false;
    }
  }
  if (event.prev_event_hash !== expectedPrev) return false;

  let eventBytes: Buffer;
  try {
    eventBytes = canonicalBytes(withoutSignature(event));
  } catch {
    return false;
  }

  return verifySignatureSafe(publicKey, eventBytes, event.signature);
}

export function evaluateConformanceVector(vector: Obj, publicKey: KeyObject): ConformanceResult {
  const inputs = vector.inputs;
  if (!isObject(inputs)) throw new Error("vector.inputs must be an object");

  const envelope = inputs.intent_envelope;
  const action = inputs.proposed_action;
  const token = inputs.decision_token;

  if (!isObject(envelope)) throw new Error("inputs.intent_envelope must be an object");
  if (!isObject(action)) throw new Error("inputs.proposed_action must be an object");
  if (token !== null && token !== undefined && !isObject(token)) {
    throw new Error("inputs.decision_token must be an object when present");
  }

  let context = vector.policy_context;
  if (!isObject(context)) context = inputs.policy_context;
  if (!isObject(context)) context = {};

  const canonicalAction = canonicalBytes(action);
  const actionHash = sha256Hex(canonicalAction);
  const canonicalEnvelope = canonicalBytes(withoutSignature(envelope));
  const canonicalToken = isObject(token) ? canonicalBytes(withoutSignature(token)) : undefined;

  const makeResult = (
    decision: Decision,
    reason: string | null,
    extra: Partial<Pick<ConformanceResult,
      "envelope_signature_valid" |
      "token_signature_valid" |
      "provenance_valid" |
      "provenance_event_id">> = {}
  ): ConformanceResult => ({
    decision,
    reason_codes: reason ? [reason] : [],
    action_hash: actionHash,
    canonical_action: canonicalAction,
    canonical_envelope: canonicalEnvelope,
    canonical_token: canonicalToken,
    ...extra,
  });

  const now = clock(context, envelope);

  if (Array.isArray(vector._duplicate_json_keys) && vector._duplicate_json_keys.length > 0) {
    return makeResult("DENY", "INVALID_ACTION");
  }

  const checkpointState = inputs.checkpoint_state;
  if (isObject(checkpointState)) {
    const createdAt = checkpointState.created_at;
    const timeout = checkpointState.timeout_seconds ?? context.checkpoint_timeout_seconds;
    if (
      typeof createdAt === "number" && Number.isInteger(createdAt) &&
      typeof timeout === "number" && Number.isInteger(timeout) &&
      now > createdAt + timeout
    ) return makeResult("DENY", "CHECKPOINT_TIMEOUT");
  }

  const checkpoint = inputs.checkpoint;
  if (isObject(checkpoint)) {
    const state = checkpoint.state ?? "";
    const expiresAt = checkpoint.expires_at;

    if (
      state === "OPEN" &&
      typeof expiresAt === "number" &&
      Number.isInteger(expiresAt) &&
      now > expiresAt
    ) return makeResult("DENY", "CHECKPOINT_TIMEOUT");

    if (state === "OPEN") return makeResult("CHECKPOINT", "CHECKPOINT_REQUIRED");
    if (state === "RESOLVED_DENY") return makeResult("DENY", "CHECKPOINT_DENIED");

    if (state === "RESOLVED_ALLOW") {
      if (checkpoint.resolver_principal_kind !== "human") {
        return makeResult("DENY", "HUMAN_RESOLUTION_REQUIRED");
      }
    } else if (state !== "" && state !== null && state !== undefined) {
      return makeResult("DENY", "INVALID_CHECKPOINT_STATE");
    }
  }

  const revokedKeys = asList(context.revoked_keys);
  if (contains(revokedKeys, envelope.signer_key_id)) return makeResult("DENY", "KEY_REVOKED");
  if (isObject(token) && contains(revokedKeys, token.signer_key_id)) {
    return makeResult("DENY", "KEY_REVOKED");
  }

  const trustedKeys = context.trusted_keys;
  if (trustedKeys !== undefined && trustedKeys !== null) {
    if (!contains(trustedKeys, envelope.signer_key_id)) {
      return makeResult("DENY", "SIGNATURE_FAILURE");
    }
    if (isObject(token) && !contains(trustedKeys, token.signer_key_id)) {
      return makeResult("DENY", "SIGNATURE_FAILURE");
    }
  }

  if (String(envelope.signer_key_id ?? "").toLowerCase().includes("hmac")) {
    return makeResult("DENY", "SIGNATURE_FAILURE");
  }

  if (isObject(token)) {
    if (String(token.signer_key_id ?? "").toLowerCase().includes("hmac")) {
      return makeResult("DENY", "SIGNATURE_FAILURE");
    }
    if (contains(context.revoked_tokens, token.token_id)) return makeResult("DENY", "TOKEN_REVOKED");

    const tokenExp = token.expires_at;
    if (
      typeof tokenExp === "number" &&
      Number.isInteger(tokenExp) &&
      now > tokenExp
    ) return makeResult("DENY", "TOKEN_EXPIRED");

    if (token.envelope_id !== envelope.envelope_id) {
      return makeResult("DENY", "TOKEN_ENVELOPE_MISMATCH");
    }
    if (token.action_hash !== actionHash) return makeResult("DENY", "HASH_MISMATCH");
  }

  if (inputs.omit_provenance === true) {
    return makeResult("DENY", "TRACEABILITY_MISSING");
  }

  const provenance = inputs.provenance_event;
  const prior = inputs.prior_provenance_event;
  let provenanceValid: boolean | undefined;
  let provenanceEventId: string | undefined;

  if (provenance !== null && provenance !== undefined) {
    if (!isObject(provenance)) return makeResult("DENY", "TRACEABILITY_FAILURE");

    provenanceValid = verifyProvenance(
      provenance,
      isObject(prior) ? prior : null,
      publicKey
    );
    if (!provenanceValid) {
      return makeResult("DENY", "TRACEABILITY_FAILURE", {
        provenance_valid: false,
      });
    }

    if (typeof provenance.event_id === "string") {
      provenanceEventId = provenance.event_id;
    }
  }

  const expiresAt = envelope.expires_at;
  if (
    typeof expiresAt === "number" &&
    Number.isInteger(expiresAt) &&
    now > expiresAt
  ) return makeResult("DENY", "ENVELOPE_EXPIRED");

  if (contains(context.revoked_envelopes, envelope.envelope_id)) {
    return makeResult("DENY", "ENVELOPE_REVOKED");
  }

  const parentId = envelope.parent_envelope_id;
  if (parentId && contains(context.revoked_envelopes, parentId)) {
    return makeResult("DENY", "ENVELOPE_REVOKED");
  }

  const budget = envelope.autonomy_budget;
  if (isObject(budget)) {
    const maxActions = budget.max_actions;
    const currentCount = context.current_action_count ?? 0;
    if (
      typeof maxActions === "number" &&
      Number.isInteger(maxActions) &&
      typeof currentCount === "number" &&
      Number.isInteger(currentCount) &&
      currentCount >= maxActions
    ) return makeResult("DENY", "BUDGET_EXHAUSTED");
  }

  if (
    contains(context.human_required_verbs, action.verb) &&
    envelope.principal_kind === "system" &&
    !envelope.parent_envelope_id
  ) return makeResult("CHECKPOINT", "HUMAN_REQUIRED");

  const failure = scopeReason(action, envelope);
  if (failure !== null) return makeResult("DENY", failure);

  const envelopeSigOk = verifySignatureSafe(publicKey, canonicalEnvelope, envelope.signature);
  if (!envelopeSigOk) {
    return makeResult("DENY", "SIGNATURE_FAILURE", {
      envelope_signature_valid: false,
      provenance_valid: provenanceValid,
    });
  }

  let tokenSigOk: boolean | undefined;
  if (isObject(token)) {
    tokenSigOk = verifySignatureSafe(
      publicKey,
      canonicalToken ?? Buffer.alloc(0),
      token.signature
    );
    if (!tokenSigOk) {
      return makeResult("DENY", "SIGNATURE_FAILURE", {
        envelope_signature_valid: true,
        token_signature_valid: false,
        provenance_valid: provenanceValid,
      });
    }
  }

  return makeResult("ALLOW", null, {
    envelope_signature_valid: true,
    token_signature_valid: tokenSigOk,
    provenance_valid: provenanceValid,
    provenance_event_id: provenanceEventId,
  });
}

export function loadVectorFile(filePath: string): LoadedVector {
  const raw = fs.readFileSync(filePath, "utf-8");
  const duplicateKeys = findDuplicateJsonKeys(raw);
  const parsed = JSON.parse(raw) as LoadedVector;
  if (duplicateKeys.length > 0) parsed._duplicate_json_keys = duplicateKeys;
  return parsed;
}

export function findDuplicateJsonKeys(text: string): string[] {
  let i = 0;
  const duplicates = new Set<string>();

  const fail = (msg: string): never => { throw new Error(`invalid JSON at ${i}: ${msg}`); };
  const ws = (): void => { while (i < text.length && /\s/.test(text[i])) i++; };

  const str = (): string => {
    if (text[i] !== '"') fail("expected string");
    const start = i++;
    while (i < text.length) {
      const ch = text[i++];
      if (ch === '"') return JSON.parse(text.slice(start, i));
      if (ch === "\\") {
        if (i >= text.length) fail("unterminated escape");
        const esc = text[i++];
        if (esc === "u") {
          if (i + 4 > text.length) fail("short unicode escape");
          i += 4;
        }
      }
    }
    return fail("unterminated string");
  };

  const number = (): void => {
    const m = text
      .slice(i)
      .match(/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/);

    if (m === null) {
      throw new Error(`invalid JSON at ${i}: invalid number`);
    }

    i += m[0].length;
  };

  const literal = (lit: string): void => {
    if (text.slice(i, i + lit.length) !== lit) fail(`expected ${lit}`);
    i += lit.length;
  };

  const value = (): void => {
    ws();
    const ch = text[i];
    if (ch === "{") return object();
    if (ch === "[") return array();
    if (ch === '"') { str(); return; }
    if (ch === "t") return literal("true");
    if (ch === "f") return literal("false");
    if (ch === "n") return literal("null");
    number();
  };

  const object = (): void => {
    if (text[i] !== "{") fail("expected object");
    i++; ws();
    const keys = new Set<string>();
    if (text[i] === "}") { i++; return; }

    while (true) {
      ws();
      const key = str();
      if (keys.has(key)) duplicates.add(key);
      keys.add(key);

      ws();
      if (text[i] !== ":") fail("expected ':'");
      i++;
      value();
      ws();

      if (text[i] === "}") { i++; return; }
      if (text[i] !== ",") fail("expected ','");
      i++;
    }
  };

  const array = (): void => {
    if (text[i] !== "[") fail("expected array");
    i++; ws();
    if (text[i] === "]") { i++; return; }

    while (true) {
      value();
      ws();
      if (text[i] === "]") { i++; return; }
      if (text[i] !== ",") fail("expected ','");
      i++;
    }
  };

  ws();
  value();
  ws();
  if (i !== text.length) fail("trailing content");
  return [...duplicates];
}
