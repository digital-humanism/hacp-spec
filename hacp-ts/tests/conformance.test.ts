import * as assert from "node:assert/strict";
import * as fs from "node:fs";
import * as path from "node:path";
import { test } from "node:test";

import { evaluateConformanceVector, loadVectorFile } from "../src/conformance";
import { loadPublicKey } from "../src/crypto";

const TEST_PUB_HEX =
  "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

function specRoot(): string {
  return process.env.HACP_SPEC_REPO
    ? path.resolve(process.env.HACP_SPEC_REPO)
    : path.resolve(process.cwd(), "..");
}

function vectorFiles(): string[] {
  const dir = path.join(specRoot(), "vectors");
  assert.ok(fs.existsSync(dir), `vectors not found: ${dir}`);

  return fs.readdirSync(dir)
    .filter((name) => name.endsWith(".json"))
    .sort()
    .map((name) => path.join(dir, name));
}

test("vector inventory is exactly 38", () => {
  assert.equal(vectorFiles().length, 38);
});

for (const filePath of vectorFiles()) {
  const vector = loadVectorFile(filePath);
  const id = String(vector.vector_id ?? path.basename(filePath));

  test(`conformance ${id}`, () => {
    assert.notEqual(vector.draft_mode, true, `${id} still has draft_mode=true`);

    const actual = evaluateConformanceVector(vector, loadPublicKey(TEST_PUB_HEX));
    const expected = vector.expected ?? {};

    assert.equal(actual.decision, expected.outcome, `${id}: decision mismatch`);

    if (Array.isArray(expected.reason_codes)) {
      assert.deepEqual(
        actual.reason_codes,
        expected.reason_codes,
        `${id}: reason_codes mismatch`
      );
    }

    assert.match(actual.action_hash, /^[0-9a-f]{64}$/);
  });
}
