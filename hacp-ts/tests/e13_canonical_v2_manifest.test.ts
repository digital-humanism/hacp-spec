import * as assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { canonicalizeV2 } from "../src/canonical_v2";

const EXPECTED_LENGTH = 1642;
const EXPECTED_SHA256 =
  "45c486b9ad1cb7a7a958f9876eb41c474cb62f144ee2ef947872c13a4ea91bb7";

interface E13Row {
  id: string;
  bits: string;
  expected: string;
}

function loadE13Manifest(): E13Row[] {
  const manifestPath = process.env.HACP_E13_CORPUS;

  assert.ok(
    manifestPath,
    "HACP_E13_CORPUS must be set to the absolute E13 manifest path",
  );

  const data = readFileSync(manifestPath);

  assert.equal(
    data.length,
    EXPECTED_LENGTH,
    "E13 corpus manifest byte length mismatch",
  );

  const actualHash = createHash("sha256").update(data).digest("hex");

  assert.equal(
    actualHash,
    EXPECTED_SHA256,
    "E13 corpus manifest SHA-256 mismatch",
  );

  const rows = JSON.parse(data.toString("utf8")) as E13Row[];

  assert.ok(Array.isArray(rows));
  assert.equal(rows.length, 24);

  for (let i = 0; i < rows.length; i++) {
    assert.equal(rows[i].id, `R${String(i + 1).padStart(2, "0")}`);
  }

  return rows;
}

function binary64FromHex(bits: string): number {
  assert.match(bits, /^[0-9a-fA-F]{16}$/);

  const buffer = Buffer.from(bits, "hex");

  assert.equal(buffer.length, 8);

  return buffer.readDoubleBE(0);
}

test("E13 canonical-v2 R01-R24 matches shared manifest", () => {
  const rows = loadE13Manifest();

  for (const row of rows) {
    const value = binary64FromHex(row.bits);

    assert.ok(
      Number.isFinite(value),
      `${row.id} must represent finite binary64`,
    );

    const actual = canonicalizeV2(value);

    assert.equal(
      actual,
      row.expected,
      `${row.id} canonical-v2 mismatch for bits ${row.bits}`,
    );
  }
});