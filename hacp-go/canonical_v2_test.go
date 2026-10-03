package main

import (
	"encoding/json"
	"testing"
)

func requireCanonicalV2(
	t *testing.T,
	input interface{},
	expected string,
) {
	t.Helper()

	got, err := CanonicalizeV2(input)
	if err != nil {
		t.Fatalf("CanonicalizeV2 returned error: %v", err)
	}

	if string(got) != expected {
		t.Fatalf(
			"CanonicalizeV2 mismatch:`nexpected: %s`nactual:   %s",
			expected,
			string(got),
		)
	}
}

func TestCanonicalV2F1FiniteFractionalNumber(t *testing.T) {
	requireCanonicalV2(
		t,
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"x": json.Number("1.5"),
			},
		},
		`{"constraints":{"x":1.5}}`,
	)
}

func TestCanonicalV2F2NullObjectMemberOmitted(t *testing.T) {
	requireCanonicalV2(
		t,
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"x": nil,
			},
		},
		`{"constraints":{}}`,
	)
}

func TestCanonicalV2F3MixedObject(t *testing.T) {
	requireCanonicalV2(
		t,
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"a": json.Number("1"),
				"b": nil,
				"c": json.Number("1.5"),
			},
		},
		`{"constraints":{"a":1,"c":1.5}}`,
	)
}

func TestCanonicalV2F4NullArrayElementPreserved(t *testing.T) {
	requireCanonicalV2(
		t,
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
	)
}

func TestCanonicalV2LeadingNullObjectMemberOmitted(t *testing.T) {
	requireCanonicalV2(
		t,
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"a": nil,
				"b": json.Number("1"),
				"c": json.Number("2"),
			},
		},
		`{"constraints":{"b":1,"c":2}}`,
	)
}

func TestCanonicalV2TrailingNullObjectMemberOmitted(t *testing.T) {
	requireCanonicalV2(
		t,
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"a": json.Number("1"),
				"b": json.Number("2"),
				"c": nil,
			},
		},
		`{"constraints":{"a":1,"b":2}}`,
	)
}

func TestCanonicalV2NestedNullObjectMemberOmitted(t *testing.T) {
	requireCanonicalV2(
		t,
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"outer": map[string]interface{}{
					"keep": json.Number("1"),
					"omit": nil,
				},
			},
		},
		`{"constraints":{"outer":{"keep":1}}}`,
	)
}

func TestCanonicalV2RejectsNonFiniteNumber(t *testing.T) {
	_, err := CanonicalizeV2(
		map[string]interface{}{
			"constraints": map[string]interface{}{
				"x": json.Number("NaN"),
			},
		},
	)

	if err == nil {
		t.Fatal("expected non-finite number rejection")
	}
}
