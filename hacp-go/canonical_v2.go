package main

import (
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

// CanonicalizeV2 produces canonical-v2 bytes.
//
// Historical canonical-v1 remains in canonical.go and is not modified or
// redirected. canonical-v2 differs only where explicitly required by the
// successor contract.
func CanonicalizeV2(v interface{}) ([]byte, error) {
	switch t := v.(type) {
	case nil:
		return []byte("null"), nil

	case bool:
		if t {
			return []byte("true"), nil
		}
		return []byte("false"), nil

	case json.Number:
		return canonicalNumberV2(t)

	case string:
		return []byte(canonicalStringV2(t)), nil

	case []interface{}:
		parts := make([]string, 0, len(t))

		for _, item := range t {
			b, err := CanonicalizeV2(item)
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
			vb, err := CanonicalizeV2(t[k])
			if err != nil {
				return nil, err
			}

			parts = append(
				parts,
				canonicalStringV2(k)+":"+string(vb),
			)
		}

		return []byte("{" + strings.Join(parts, ",") + "}"), nil

	default:
		return nil, fmt.Errorf("unsupported type: %T", v)
	}
}

func canonicalNumberV2(n json.Number) ([]byte, error) {
	s := n.String()

	if isIntV2(s) {
		return []byte(s), nil
	}

	f, err := n.Float64()
	if err != nil {
		return nil, err
	}

	if math.IsNaN(f) || math.IsInf(f, 0) {
		return nil, fmt.Errorf("non-finite numbers are not valid canonical JSON numbers")
	}

	return []byte(strconv.FormatFloat(f, 'g', -1, 64)), nil
}

func isIntV2(s string) bool {
	if s == "" {
		return false
	}

	i := 0

	if s[0] == '-' || s[0] == '+' {
		i = 1
	}

	if i == len(s) {
		return false
	}

	for ; i < len(s); i++ {
		if s[i] < '0' || s[i] > '9' {
			return false
		}
	}

	return true
}

func canonicalStringV2(s string) string {
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
