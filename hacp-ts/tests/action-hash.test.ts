import * as assert from "node:assert/strict";
import { test } from "node:test";

import { canonicalBytes } from "../src/canonical";
import { sha256Hex } from "../src/crypto";

const hash = (v: Record<string, unknown>): string => sha256Hex(canonicalBytes(v));

test("action hash is independent of object field order", () => {
  const a = {
    verb: "read",
    resource_class: "document",
    audience: "internal",
    reversibility: "reversible",
    externality: "internal",
    data_class: "internal",
  };
  const b = {
    data_class: "internal",
    externality: "internal",
    reversibility: "reversible",
    audience: "internal",
    resource_class: "document",
    verb: "read",
  };
  assert.equal(hash(a), hash(b));
});

for (const [name, patch] of [
  ["audience-external", { audience: "external" }],
  ["reversibility-irreversible", { reversibility: "irreversible" }],
  ["externality-external", { externality: "external" }],
  ["data_class-confidential", { data_class: "confidential" }],
] as const) {
  test(`action hash changes: ${name}`, () => {
    const base = {
      verb: "read",
      resource_class: "document",
      audience: "internal",
      reversibility: "reversible",
      externality: "internal",
      data_class: "internal",
    };
    assert.notEqual(hash(base), hash({ ...base, ...patch }));
  });
}
