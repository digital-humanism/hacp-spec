import * as assert from "node:assert/strict";
import { test } from "node:test";

import { canonicalizeV2 } from "../src/canonical_v2";

test("F1: finite fractional number is preserved", () => {
  assert.equal(
    canonicalizeV2({ constraints: { x: 1.5 } }),
    '{"constraints":{"x":1.5}}',
  );
});

test("F2: null-valued object member is omitted", () => {
  assert.equal(
    canonicalizeV2({ constraints: { x: null } }),
    '{"constraints":{}}',
  );
});

test("F3: mixed object omits null and preserves remaining members", () => {
  assert.equal(
    canonicalizeV2({ constraints: { a: 1, b: null, c: 1.5 } }),
    '{"constraints":{"a":1,"c":1.5}}',
  );
});

test("F4: null array element is preserved", () => {
  assert.equal(
    canonicalizeV2({ constraints: { x: [1, null, 2] } }),
    '{"constraints":{"x":[1,null,2]}}',
  );
});

test("leading null-valued object member is omitted", () => {
  assert.equal(
    canonicalizeV2({ constraints: { a: null, b: 1, c: 2 } }),
    '{"constraints":{"b":1,"c":2}}',
  );
});

test("trailing null-valued object member is omitted", () => {
  assert.equal(
    canonicalizeV2({ constraints: { a: 1, b: 2, c: null } }),
    '{"constraints":{"a":1,"b":2}}',
  );
});

test("nested null-valued object member is omitted", () => {
  assert.equal(
    canonicalizeV2({
      constraints: {
        outer: {
          keep: 1,
          omit: null,
        },
      },
    }),
    '{"constraints":{"outer":{"keep":1}}}',
  );
});

test("non-finite numbers are rejected", () => {
  assert.throws(
    () => canonicalizeV2({ constraints: { x: Number.POSITIVE_INFINITY } }),
    /Non-finite numbers/,
  );
});
