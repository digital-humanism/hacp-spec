package main

import (
	"encoding/json"
	"math"
	"strconv"
	"testing"
)

func requireCanonicalV2R2(
	t *testing.T,
	input interface{},
	expected string,
) {
	t.Helper()

	got, err := CanonicalizeV2R2(input)
	if err != nil {
		t.Fatalf("CanonicalizeV2R2 returned error: %v", err)
	}

	if string(got) != expected {
		t.Fatalf(
			"CanonicalizeV2R2 mismatch:\nexpected: %s\nactual:   %s",
			expected,
			string(got),
		)
	}
}

func TestCanonicalV2R2PreservesOriginalContract(t *testing.T) {
	cases := []struct {
		input    interface{}
		expected string
	}{
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"x": json.Number("1.5"),
				},
			},
			`{"constraints":{"x":1.5}}`,
		},
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"x": nil,
				},
			},
			`{"constraints":{}}`,
		},
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"a": json.Number("1"),
					"b": nil,
					"c": json.Number("1.5"),
				},
			},
			`{"constraints":{"a":1,"c":1.5}}`,
		},
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"x": []interface{}{
						json.Number("1"),
						nil,
						json.Number("2"),
					},
				},
			},
			`{"constraints":{"x":[1,null,2]}}`,
		},
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"a": nil,
					"b": json.Number("1"),
					"c": json.Number("2"),
				},
			},
			`{"constraints":{"b":1,"c":2}}`,
		},
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"a": json.Number("1"),
					"b": json.Number("2"),
					"c": nil,
				},
			},
			`{"constraints":{"a":1,"b":2}}`,
		},
		{
			map[string]interface{}{
				"constraints": map[string]interface{}{
					"outer": map[string]interface{}{
						"keep": json.Number("1"),
						"omit": nil,
					},
				},
			},
			`{"constraints":{"outer":{"keep":1}}}`,
		},
	}

	for i, tc := range cases {
		t.Run(strconv.Itoa(i+1), func(t *testing.T) {
			requireCanonicalV2R2(t, tc.input, tc.expected)
		})
	}
}

func TestCanonicalV2R2RFC8785AppendixBNumericCorpus(t *testing.T) {
	cases := []struct {
		hex      string
		expected string
	}{
		{"0000000000000000", "0"},
		{"8000000000000000", "0"},
		{"0000000000000001", "5e-324"},
		{"8000000000000001", "-5e-324"},
		{"7fefffffffffffff", "1.7976931348623157e+308"},
		{"ffefffffffffffff", "-1.7976931348623157e+308"},
		{"4340000000000000", "9007199254740992"},
		{"c340000000000000", "-9007199254740992"},
		{"4430000000000000", "295147905179352830000"},
		{"44b52d02c7e14af5", "9.999999999999997e+22"},
		{"44b52d02c7e14af6", "1e+23"},
		{"44b52d02c7e14af7", "1.0000000000000001e+23"},
		{"444b1ae4d6e2ef4e", "999999999999999700000"},
		{"444b1ae4d6e2ef4f", "999999999999999900000"},
		{"444b1ae4d6e2ef50", "1e+21"},
		{"3eb0c6f7a0b5ed8c", "9.999999999999997e-7"},
		{"3eb0c6f7a0b5ed8d", "0.000001"},
		{"41b3de4355555553", "333333333.3333332"},
		{"41b3de4355555554", "333333333.33333325"},
		{"41b3de4355555555", "333333333.3333333"},
		{"41b3de4355555556", "333333333.3333334"},
		{"41b3de4355555557", "333333333.33333343"},
		{"becbf647612f3696", "-0.0000033333333333333333"},
		{"43143ff3c1cb0959", "1424953923781206.2"},
	}

	for _, tc := range cases {
		t.Run(tc.hex, func(t *testing.T) {
			bits, err := strconv.ParseUint(tc.hex, 16, 64)
			if err != nil {
				t.Fatalf("invalid IEEE-754 fixture bits: %v", err)
			}

			value := math.Float64frombits(bits)

			input :=
				json.Number(
					strconv.FormatFloat(
						value,
						'g',
						-1,
						64,
					),
				)

			requireCanonicalV2R2(
				t,
				input,
				tc.expected,
			)
		})
	}
}

func TestCanonicalV2R2RejectsNonFiniteNumber(t *testing.T) {
	for _, input := range []json.Number{
		json.Number("NaN"),
		json.Number("Inf"),
		json.Number("+Inf"),
		json.Number("-Inf"),
	} {
		if _, err := CanonicalizeV2R2(input); err == nil {
			t.Fatalf(
				"expected non-finite rejection for %q",
				input.String(),
			)
		}
	}
}
