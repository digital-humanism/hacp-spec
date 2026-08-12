import * as crypto from "crypto";

// DER SPKI prefix for a raw 32-byte Ed25519 public key.
const SPKI_PREFIX = Buffer.from("302a300506032b6570032100", "hex");

export function sha256Hex(data: Buffer): string {
  return crypto.createHash("sha256").update(data).digest("hex");
}

export function loadPublicKey(hexKey: string): crypto.KeyObject {
  const raw = Buffer.from(hexKey, "hex");
  if (raw.length !== 32) {
    throw new Error(`public key must be 32 bytes, got ${raw.length}`);
  }
  const spki = Buffer.concat([SPKI_PREFIX, raw]);
  return crypto.createPublicKey({ key: spki, format: "der", type: "spki" });
}

export function verifySignature(
  pub: crypto.KeyObject,
  payload: Buffer,
  sigB64Url: string
): boolean {
  try {
    const sig = Buffer.from(sigB64Url, "base64url");
    return crypto.verify(null, payload, pub, sig);
  } catch {
    return false;
  }
}