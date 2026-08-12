"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.evaluate = evaluate;
exports.checkpointDecision = checkpointDecision;
const getStr = (m, k) => typeof m?.[k] === "string" ? m[k] : "";
const getMap = (m, k) => m?.[k] && typeof m[k] === "object" && !Array.isArray(m[k]) ? m[k] : null;
const getInt = (m, k) => typeof m?.[k] === "number" && Number.isInteger(m[k]) ? m[k] : null;
const inList = (list, val) => Array.isArray(list) && list.some((x) => x === val);
// HACP evaluate() policy logic per spec.
function evaluate(action, envelope, context, token) {
    // Clock
    const currentTime = getInt(context, "current_time") ??
        getInt(context, "clock") ??
        getInt(envelope, "issued_at") ??
        0;
    // Envelope expiry
    const envExp = getInt(envelope, "expires_at");
    if (envExp !== null && currentTime > envExp)
        return "DENY";
    // Envelope revocation (incl. parent inheritance)
    if (inList(context["revoked_envelopes"], getStr(envelope, "envelope_id")))
        return "DENY";
    if (inList(context["revoked_envelopes"], getStr(envelope, "parent_envelope_id")))
        return "DENY";
    // Key revocation
    if (inList(context["revoked_keys"], getStr(envelope, "signer_key_id")))
        return "DENY";
    if (token && inList(context["revoked_keys"], getStr(token, "signer_key_id")))
        return "DENY";
    // Token revocation + expiry
    if (token) {
        if (inList(context["revoked_tokens"], getStr(token, "token_id")))
            return "DENY";
        const tokExp = getInt(token, "expires_at");
        if (tokExp !== null && currentTime > tokExp)
            return "DENY";
    }
    // Trusted keys
    const trusted = context["trusted_keys"];
    if (Array.isArray(trusted)) {
        if (!inList(trusted, getStr(envelope, "signer_key_id")))
            return "DENY";
        if (token && !inList(trusted, getStr(token, "signer_key_id")))
            return "DENY";
    }
    // Reject non-Ed25519 algorithms
    if (getStr(envelope, "signer_key_id").toLowerCase().includes("hmac"))
        return "DENY";
    if (token && getStr(token, "signer_key_id").toLowerCase().includes("hmac"))
        return "DENY";
    // Bounded autonomy (INV-7)
    const budget = getMap(envelope, "autonomy_budget");
    if (budget) {
        const maxActions = getInt(budget, "max_actions");
        if (maxActions !== null) {
            const current = getInt(context, "current_action_count") ?? 0;
            if (current >= maxActions)
                return "DENY";
        }
    }
    // Human final decision (INV-1)
    if (inList(context["human_required_verbs"], getStr(action, "verb"))) {
        if (getStr(envelope, "principal_kind") === "system" &&
            getStr(envelope, "parent_envelope_id") === "") {
            return "CHECKPOINT";
        }
    }
    // Boundary re-authorization (INV-2)
    const scope = getMap(envelope, "scope");
    if (!scope)
        return "DENY";
    const checks = [
        ["audience", "audiences"],
        ["reversibility", "reversibility"],
        ["externality", "externality"],
        ["data_class", "data_classes"],
        ["verb", "verbs"],
        ["resource_class", "resource_classes"],
    ];
    for (const [attr, key] of checks) {
        if (Array.isArray(scope[key]) && !inList(scope[key], getStr(action, attr)))
            return "DENY";
    }
    // Quantity
    const q = getInt(action, "quantity");
    const maxQ = getInt(scope, "max_quantity");
    if (q !== null && maxQ !== null && q > maxQ)
        return "DENY";
    // Destination allowlist (absent optional => UNKNOWN_ATTRIBUTE)
    if (Array.isArray(scope["destinations"]) && scope["destinations"].length > 0) {
        const dest = getStr(action, "destination");
        if (!dest || !inList(scope["destinations"], dest))
            return "DENY";
    }
    // Tool allowlist (absent optional => UNKNOWN_ATTRIBUTE)
    if (Array.isArray(scope["tool_names"]) && scope["tool_names"].length > 0) {
        const tool = getStr(action, "tool_name");
        if (!tool || !inList(scope["tool_names"], tool))
            return "DENY";
    }
    return "ALLOW";
}
function checkpointDecision(cp, ctx) {
    const clock = getInt(ctx, "clock") ?? getInt(ctx, "current_time") ?? 0;
    let state = cp.state;
    const exp = getInt(cp, "expires_at");
    if (state === "OPEN" && exp !== null && clock > exp)
        state = "EXPIRED";
    if (state === "EXPIRED")
        return "DENY";
    if (state === "RESOLVED_DENY")
        return "DENY";
    if (state === "OPEN")
        return "CHECKPOINT";
    if (state === "RESOLVED_ALLOW") {
        if (cp.resolver_principal_kind !== "human")
            return "DENY";
        return null;
    }
    return "DENY";
}
