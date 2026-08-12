package main

import (
	"encoding/json"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

// Canonicalize produces RFC 8785 (JCS) canonical bytes.
func Canonicalize(v interface{}) ([]byte, error) {
	switch t := v.(type) {
	case nil:
		return []byte("null"), nil
	case bool:
		if t {
			return []byte("true"), nil
		}
		return []byte("false"), nil
	case json.Number:
		return canonicalNumber(t)
	case string:
		return []byte(canonicalString(t)), nil
	case []interface{}:
		var parts []string
		for _, item := range t {
			b, err := Canonicalize(item)
			if err != nil {
				return nil, err
			}
			parts = append(parts, string(b))
		}
		return []byte("[" + strings.Join(parts, ",") + "]"), nil
	case map[string]interface{}:
		keys := make([]string, 0, len(t))
		for k := range t {
			keys = append(keys, k)
		}
		sort.Strings(keys)
		var parts []string
		for _, k := range keys {
			vb, err := Canonicalize(t[k])
			if err != nil {
				return nil, err
			}
			parts = append(parts, canonicalString(k)+":"+string(vb))
		}
		return []byte("{" + strings.Join(parts, ",") + "}"), nil
	default:
		return nil, fmt.Errorf("unsupported type: %T", v)
	}
}

func canonicalNumber(n json.Number) ([]byte, error) {
	s := n.String()
	if isInt(s) {
		return []byte(s), nil
	}
	f, err := n.Float64()
	if err != nil {
		return nil, err
	}
	return []byte(strconv.FormatFloat(f, 'g', -1, 64)), nil
}

func isInt(s string) bool {
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

func canonicalString(s string) string {
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