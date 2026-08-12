// RFC 8785 (JCS) canonicalization.

export function canonicalize(v: unknown): string {
  if (v === null || v === undefined) return "null";
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "number") return canonicalNumber(v);
  if (typeof v === "string") return canonicalString(v);
  if (Array.isArray(v)) {
    return "[" + v.map(canonicalize).join(",") + "]";
  }
  if (typeof v === "object") {
    const obj = v as Record<string, unknown>;
    const keys = Object.keys(obj).sort();
    const parts = keys.map((k) => canonicalString(k) + ":" + canonicalize(obj[k]));
    return "{" + parts.join(",") + "}";
  }
  throw new Error(`Unsupported type: ${typeof v}`);
}

function canonicalNumber(n: number): string {
  // Integers serialize as-is (matches JCS for safe integers).
  return String(n);
}

function canonicalString(s: string): string {
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

export function canonicalBytes(v: unknown): Buffer {
  return Buffer.from(canonicalize(v), "utf-8");
}