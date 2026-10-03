import assert from "node:assert/strict";
import fs from "node:fs";
import test from "node:test";

import { canonicalBytesV2 } from "../src/canonical_v2";
import {
  loadPublicKey,
  sha256Hex,
  verifySignature,
} from "../src/crypto";

const EXPECTED_LENGTH = 423;

const EXPECTED_SHA256 =
  "15edb6e1f8cb9124f282a0d1fc54118ace6ee3c43a1d2671553401a20e2c0ed1";

const EXPECTED_PUBLIC_KEY_HEX =
  "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

const IMMUTABLE_SIGNATURE =
  "_5TKPo_xMVHrR-ucaVxKuysTuZaPnoSQgG7o7YVLyzYQW9NlwrjZePdNnlP1D2VgAdhwwJDJTLKYN6XP2I45Cw";

test("E13 Family 2 hash/signature binding", () => {
  const fixturePath = process.env.HACP_E13_FAMILY2_FIXTURE;
  assert.ok(fixturePath, "HACP_E13_FAMILY2_FIXTURE must be set");

  const publicKeyPath = process.env.HACP_E13_FAMILY2_PUBLIC_KEY;
  assert.ok(publicKeyPath, "HACP_E13_FAMILY2_PUBLIC_KEY must be set");

  const fixture = JSON.parse(
    fs.readFileSync(fixturePath, "utf8"),
  ) as unknown;

  const canonical = canonicalBytesV2(fixture);

  assert.equal(
    canonical.length,
    EXPECTED_LENGTH,
    "V5 canonical byte length mismatch",
  );

  assert.equal(
    sha256Hex(canonical),
    EXPECTED_SHA256,
    "V5 SHA-256 mismatch",
  );

  const publicKeyHex =
    fs.readFileSync(publicKeyPath, "utf8").trim();

  assert.equal(
    publicKeyHex,
    EXPECTED_PUBLIC_KEY_HEX,
    "public-key oracle mismatch",
  );

  const publicKey = loadPublicKey(publicKeyHex);

  assert.equal(
    verifySignature(
      publicKey,
      canonical,
      IMMUTABLE_SIGNATURE,
    ),
    true,
    "V6 positive immutable-signature verification failed",
  );

  const mutated = Buffer.from(canonical);
  mutated[0] ^= 0x01;

  let changed = 0;

  for (let i = 0; i < canonical.length; i += 1) {
    if (canonical[i] !== mutated[i]) {
      changed += 1;
    }
  }

  assert.equal(
    changed,
    1,
    "E13 deterministic mutation must change exactly one byte",
  );

  assert.equal(
    verifySignature(
      publicKey,
      mutated,
      IMMUTABLE_SIGNATURE,
    ),
    false,
    "V6 mutated payload unexpectedly verified",
  );
});