"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const fs = __importStar(require("fs"));
const canonical_1 = require("./canonical");
const crypto_1 = require("./crypto");
const evaluate_1 = require("./evaluate");
// Published conformance test public key (harness/keys/KEYS.md). TEST ONLY.
const TEST_PUB_HEX = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";
const GENESIS = "0".repeat(64);
function verifyProvenance(event, prior, pub) {
    if ((0, crypto_1.sha256Hex)((0, canonical_1.canonicalBytes)(event.payload)) !== event.payload_hash)
        return false;
    const expectedPrev = prior ? (0, crypto_1.sha256Hex)((0, canonical_1.canonicalBytes)(prior)) : GENESIS;
    if (event.prev_event_hash !== expectedPrev)
        return false;
    const { signature, ...evNoSig } = event;
    return (0, crypto_1.verifySignature)(pub, (0, canonical_1.canonicalBytes)(evNoSig), signature);
}
function main() {
    const args = process.argv.slice(2);
    if (args[0] !== "evaluate") {
        console.error(JSON.stringify({
            error: "USAGE",
            message: "usage: hacp-ts evaluate --vector <path>",
        }));
        process.exit(2);
    }
    let vectorPath = "";
    let pubKeyPath = "";
    for (let i = 1; i < args.length; i++) {
        if (args[i] === "--vector") {
            vectorPath = args[++i] ?? "";
        }
        else if (args[i] === "--public-key") {
            pubKeyPath = args[++i] ?? "";
        }
    }
    if (!vectorPath) {
        console.error(JSON.stringify({ error: "USAGE", message: "--vector required" }));
        process.exit(2);
    }
    process.exit(runEvaluate(vectorPath, pubKeyPath));
}
function runEvaluate(vectorPath, pubKeyPath) {
    let root;
    try {
        root = JSON.parse(fs.readFileSync(vectorPath, "utf-8"));
    }
    catch (e) {
        console.error(JSON.stringify({ error: "PARSE_ERROR", message: e.message }));
        return 1;
    }
    const inputs = root.inputs ?? {};
    const action = inputs.proposed_action ?? {};
    const envelope = inputs.intent_envelope ?? {};
    const context = root.policy_context ?? {};
    const token = inputs.decision_token ?? null;
    let pubHex = TEST_PUB_HEX;
    if (pubKeyPath) {
        try {
            pubHex = fs.readFileSync(pubKeyPath, "utf-8").trim();
        }
        catch {
            // fallback to the published test key
        }
    }
    let pub;
    try {
        pub = (0, crypto_1.loadPublicKey)(pubHex);
    }
    catch (e) {
        console.error(JSON.stringify({ error: "KEY_ERROR", message: e.message }));
        return 1;
    }
    let decision = (0, evaluate_1.evaluate)(action, envelope, context, token);
    // Crypto verification (INV-3, INV-5)
    if (decision === "ALLOW" && token) {
        const computed = (0, crypto_1.sha256Hex)((0, canonical_1.canonicalBytes)(action));
        if (token.action_hash !== computed) {
            decision = "DENY";
        }
        else {
            const { signature, ...tokenNoSig } = token;
            if (!(0, crypto_1.verifySignature)(pub, (0, canonical_1.canonicalBytes)(tokenNoSig), signature)) {
                decision = "DENY";
            }
        }
    }
    const provenance = inputs.provenance_event ?? null;
    const prior = inputs.prior_provenance_event ?? null;
    const omit = inputs.omit_provenance === true;
    if (decision === "ALLOW") {
        if (omit) {
            decision = "DENY";
        }
        else if (provenance && !verifyProvenance(provenance, prior, pub)) {
            decision = "DENY";
        }
    }
    const resp = { decision };
    if (decision === "ALLOW" && token) {
        resp.decision_token = token;
    }
    if (decision === "ALLOW" && provenance && !omit) {
        resp.provenance_event_id = provenance.event_id;
    }
    console.log(JSON.stringify(resp));
    return 0;
}
main();
