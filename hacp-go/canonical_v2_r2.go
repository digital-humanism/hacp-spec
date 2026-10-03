package main

import (
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

// CanonicalizeV2R2 produces canonical-v2 bytes using implementation revision r2.
//
// Behavioral identity remains canonical-v2. Historical canonical-v1 and the
// pre-convergence canonical-v2 implementation remain unchanged.
func CanonicalizeV2R2(v interface{}) ([]byte, error) {
	switch t := v.(type) {
	case nil:
		return []byte("null"), nil

	case bool:
		if t {
			return []byte("true"), nil
		}
		return []byte("false"), nil

	case json.Number:
		return canonicalNumberV2R2(t)

	case string:
		return []byte(canonicalStringV2R2(t)), nil

	case []interface{}:
		parts := make([]string, 0, len(t))

		for _, item := range t {
			b, err := CanonicalizeV2R2(item)
			if err != nil {
				return nil, err
			}

			parts = append(parts, string(b))
		}

		return []byte("[" + strings.Join(parts, ",") + "]"), nil

	case map[string]interface{}:
		keys := make([]string, 0, len(t))

		for k, value := range t {
			if value == nil {
				continue
			}

			keys = append(keys, k)
		}

		sort.Strings(keys)

		parts := make([]string, 0, len(keys))

		for _, k := range keys {
			vb, err := CanonicalizeV2R2(t[k])
			if err != nil {
				return nil, err
			}

			parts = append(
				parts,
				canonicalStringV2R2(k)+":"+string(vb),
			)
		}

		return []byte("{" + strings.Join(parts, ",") + "}"), nil

	default:
		return nil, fmt.Errorf("unsupported type: %T", v)
	}
}

func canonicalNumberV2R2(n json.Number) ([]byte, error) {
	f, err := n.Float64()
	if err != nil {
		return nil, err
	}

	if math.IsNaN(f) || math.IsInf(f, 0) {
		return nil, fmt.Errorf("non-finite numbers are not valid canonical JSON numbers")
	}

	s, err := formatECMAScriptNumberV2R2(f)
	if err != nil {
		return nil, err
	}

	return []byte(s), nil
}

func formatECMAScriptNumberV2R2(value float64) (string, error) {
	if math.IsNaN(value) || math.IsInf(value, 0) {
		return "", fmt.Errorf("non-finite numbers are not valid canonical JSON numbers")
	}

	if value == 0 {
		return "0", nil
	}

	raw := strings.ToLower(strconv.FormatFloat(value, 'g', -1, 64))

	sign := ""
	unsigned := raw

	if strings.HasPrefix(unsigned, "-") {
		sign = "-"
		unsigned = unsigned[1:]
	}

	eIndex := strings.IndexByte(unsigned, 'e')

	if eIndex < 0 {
		return sign + unsigned, nil
	}

	mantissa := unsigned[:eIndex]

	exponent, err := strconv.Atoi(unsigned[eIndex+1:])
	if err != nil {
		return "", fmt.Errorf("invalid shortest-roundtrip exponent: %w", err)
	}

	decimalBefore := len(mantissa)

	if dot := strings.IndexByte(mantissa, '.'); dot >= 0 {
		decimalBefore = dot
	}

	digits := strings.ReplaceAll(mantissa, ".", "")
	digits = strings.TrimLeft(digits, "0")

	if digits == "" {
		digits = "0"
	}

	k := exponent + decimalBefore
	scientificExponent := k - 1

	if scientificExponent >= -6 && scientificExponent < 21 {
		var body string

		switch {
		case k <= 0:
			body =
				"0." +
					strings.Repeat("0", -k) +
					digits

		case k >= len(digits):
			body =
				digits +
					strings.Repeat("0", k-len(digits))

		default:
			body =
				digits[:k] +
					"." +
					digits[k:]
		}

		if strings.Contains(body, ".") {
			body = strings.TrimRight(body, "0")
			body = strings.TrimRight(body, ".")
		}

		return sign + body, nil
	}

	body := digits[:1]

	if len(digits) > 1 {
		tail := strings.TrimRight(digits[1:], "0")

		if tail != "" {
			body += "." + tail
		}
	}

	exponentSign := "+"

	if scientificExponent < 0 {
		exponentSign = "-"
		scientificExponent = -scientificExponent
	}

	return sign +
		body +
		"e" +
		exponentSign +
		strconv.Itoa(scientificExponent), nil
}

func canonicalStringV2R2(s string) string {
	var b strings.Builder

	b.WriteByte('"')

	for _, r := range s {
		switch r {
		case '"':
			b.WriteString(`\"`)
		case '\\':
			b.WriteString(`\\`)
		case '\b':
			b.WriteString(`\b`)
		case '\f':
			b.WriteString(`\f`)
		case '\n':
			b.WriteString(`\n`)
		case '\r':
			b.WriteString(`\r`)
		case '\t':
			b.WriteString(`\t`)
		default:
			if r < 0x20 {
				b.WriteString(fmt.Sprintf(`\u%04x`, r))
			} else {
				b.WriteRune(r)
			}
		}
	}

	b.WriteByte('"')

	return b.String()
}
