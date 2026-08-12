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
exports.sha256Hex = sha256Hex;
exports.loadPublicKey = loadPublicKey;
exports.verifySignature = verifySignature;
const crypto = __importStar(require("crypto"));
// DER SPKI prefix for a raw 32-byte Ed25519 public key.
const SPKI_PREFIX = Buffer.from("302a300506032b6570032100", "hex");
function sha256Hex(data) {
    return crypto.createHash("sha256").update(data).digest("hex");
}
function loadPublicKey(hexKey) {
    const raw = Buffer.from(hexKey, "hex");
    if (raw.length !== 32) {
        throw new Error(`public key must be 32 bytes, got ${raw.length}`);
    }
    const spki = Buffer.concat([SPKI_PREFIX, raw]);
    return crypto.createPublicKey({ key: spki, format: "der", type: "spki" });
}
function verifySignature(pub, payload, sigB64Url) {
    try {
        const sig = Buffer.from(sigB64Url, "base64url");
        return crypto.verify(null, payload, pub, sig);
    }
    catch {
        return false;
    }
}
