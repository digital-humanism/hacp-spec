// canonical-v2 successor surface.
// Historical canonical-v1 remains in canonical.ts and is not modified or redirected.

export function canonicalizeV2(v: unknown): string {
  if (v === null || v === undefined) return "null";
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "number") return canonicalNumberV2(v);
  if (typeof v === "string") return canonicalStringV2(v);

  if (Array.isArray(v)) {
    return "[" + v.map(canonicalizeV2).join(",") + "]";
  }

  if (typeof v === "object") {
    const obj = v as Record<string, unknown>;

    const keys = Object.keys(obj)
      .filter((k) => obj[k] !== null)
      .sort();

    const parts = keys.map(
      (k) => canonicalStringV2(k) + ":" + canonicalizeV2(obj[k]),
    );

    return "{" + parts.join(",") + "}";
  }

  throw new Error(`Unsupported type: ${typeof v}`);
}

function canonicalNumberV2(n: number): string {
  if (!Number.isFinite(n)) {
    throw new Error("Non-finite numbers are not valid canonical JSON numbers");
  }

  return String(n);
}

function canonicalStringV2(s: string): string {
  let out = '"';

  for (const ch of s) {
    const code = ch.codePointAt(0)!;

    switch (ch) {
      case '"': out += '\\"'; break;
      case "\\": out += "\\\\"; break;
      case "\b": out += "\\b"; break;
      case "\f": out += "\\f"; break;
      case "\n": out += "\\n"; break;
      case "\r": out += "\\r"; break;
      case "\t": out += "\\t"; break;
      default:
        if (code < 0x20) {
          out += "\\u" + code.toString(16).padStart(4, "0");
        } else {
          out += ch;
        }
    }
  }

  return out + '"';
}

export function canonicalBytesV2(v: unknown): Buffer {
  return Buffer.from(canonicalizeV2(v), "utf-8");
}
