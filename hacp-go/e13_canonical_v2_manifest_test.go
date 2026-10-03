package main

import (
"crypto/sha256"
"encoding/hex"
"encoding/json"
"fmt"
"math"
"os"
"strconv"
"testing"
)

const e13ExpectedManifestLength = 1642
const e13ExpectedManifestSHA256 = "45c486b9ad1cb7a7a958f9876eb41c474cb62f144ee2ef947872c13a4ea91bb7"

type e13Row struct {
ID       string `json:"id"`
Bits     string `json:"bits"`
Expected string `json:"expected"`
}

func loadE13Manifest(t *testing.T) []e13Row {
t.Helper()

path := os.Getenv("HACP_E13_CORPUS")
if path == "" {
t.Fatal("HACP_E13_CORPUS must be set to the absolute E13 manifest path")
}

data, err := os.ReadFile(path)
if err != nil {
t.Fatalf("read E13 corpus manifest: %v", err)
}

if len(data) != e13ExpectedManifestLength {
t.Fatalf(
"E13 corpus manifest byte length mismatch: expected %d got %d",
e13ExpectedManifestLength,
len(data),
)
}

sum := sha256.Sum256(data)
actualHash := hex.EncodeToString(sum[:])

if actualHash != e13ExpectedManifestSHA256 {
t.Fatalf(
"E13 corpus manifest SHA-256 mismatch: expected %s got %s",
e13ExpectedManifestSHA256,
actualHash,
)
}

var rows []e13Row

if err := json.Unmarshal(data, &rows); err != nil {
t.Fatalf("parse E13 corpus manifest: %v", err)
}

if len(rows) != 24 {
t.Fatalf("E13 corpus manifest must contain 24 rows; got %d", len(rows))
}

for i, row := range rows {
expectedID := fmt.Sprintf("R%02d", i+1)

if row.ID != expectedID {
t.Fatalf(
"E13 corpus row identifier mismatch at index %d: expected %s got %s",
i,
expectedID,
row.ID,
)
}
}

return rows
}

func TestE13CanonicalV2R01R24MatchesSharedManifest(t *testing.T) {
rows := loadE13Manifest(t)

for _, row := range rows {
row := row

t.Run(row.ID, func(t *testing.T) {
if len(row.Bits) != 16 {
t.Fatalf("%s must contain exactly 16 hexadecimal digits", row.ID)
}

bits, err := strconv.ParseUint(row.Bits, 16, 64)
if err != nil {
t.Fatalf("%s contains invalid hexadecimal bits: %v", row.ID, err)
}

value := math.Float64frombits(bits)

if math.IsNaN(value) || math.IsInf(value, 0) {
t.Fatalf("%s must represent finite binary64", row.ID)
}

input := json.Number(strconv.FormatFloat(value, 'g', -1, 64))

actual, err := CanonicalizeV2R2(input)
if err != nil {
t.Fatalf("%s canonical-v2 r2 error: %v", row.ID, err)
}

if string(actual) != row.Expected {
t.Fatalf(
"%s canonical-v2 mismatch for bits %s: expected %s got %s",
row.ID,
row.Bits,
row.Expected,
string(actual),
)
}
})
}
}