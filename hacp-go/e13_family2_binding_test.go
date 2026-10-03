package main

import (
"encoding/json"
"os"
"testing"
)

const e13Family2ExpectedLength = 423

const e13Family2ExpectedSHA256 =
"15edb6e1f8cb9124f282a0d1fc54118ace6ee3c43a1d2671553401a20e2c0ed1"

const e13Family2ExpectedPublicKeyHex =
"9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3"

const e13Family2ImmutableSignature =
"_5TKPo_xMVHrR-ucaVxKuysTuZaPnoSQgG7o7YVLyzYQW9NlwrjZePdNnlP1D2VgAdhwwJDJTLKYN6XP2I45Cw"

func TestE13Family2HashSignatureBinding(t *testing.T) {
fixturePath := os.Getenv("HACP_E13_FAMILY2_FIXTURE")
if fixturePath == "" {
t.Fatal("HACP_E13_FAMILY2_FIXTURE must be set")
}

publicKeyPath := os.Getenv("HACP_E13_FAMILY2_PUBLIC_KEY")
if publicKeyPath == "" {
t.Fatal("HACP_E13_FAMILY2_PUBLIC_KEY must be set")
}

fixtureFile, err := os.Open(fixturePath)
if err != nil {
t.Fatalf("open fixture: %v", err)
}
defer fixtureFile.Close()

decoder := json.NewDecoder(fixtureFile)
decoder.UseNumber()

var fixture interface{}
if err := decoder.Decode(&fixture); err != nil {
t.Fatalf("decode fixture: %v", err)
}

canonical, err := CanonicalizeV2R2(fixture)
if err != nil {
t.Fatalf("canonical-v2: %v", err)
}

if len(canonical) != e13Family2ExpectedLength {
t.Fatalf(
"V5 canonical byte length mismatch: got %d want %d",
len(canonical),
e13Family2ExpectedLength,
)
}

if got := SHA256Hex(canonical); got != e13Family2ExpectedSHA256 {
t.Fatalf(
"V5 SHA-256 mismatch: got %s want %s",
got,
e13Family2ExpectedSHA256,
)
}

publicKeyRaw, err := os.ReadFile(publicKeyPath)
if err != nil {
t.Fatalf("read public key: %v", err)
}

publicKeyHex := string(publicKeyRaw)
for len(publicKeyHex) > 0 {
last := publicKeyHex[len(publicKeyHex)-1]
if last == '\r' || last == '\n' || last == ' ' || last == '\t' {
publicKeyHex = publicKeyHex[:len(publicKeyHex)-1]
continue
}
break
}

if publicKeyHex != e13Family2ExpectedPublicKeyHex {
t.Fatalf("public-key oracle mismatch")
}

publicKey, err := LoadPublicKey(publicKeyHex)
if err != nil {
t.Fatalf("load public key: %v", err)
}

if !VerifySignature(
publicKey,
canonical,
e13Family2ImmutableSignature,
) {
t.Fatal("V6 positive immutable-signature verification failed")
}

mutated := append([]byte(nil), canonical...)
mutated[0] ^= 0x01

changed := 0
for i := range canonical {
if canonical[i] != mutated[i] {
changed++
}
}

if changed != 1 {
t.Fatalf(
"E13 deterministic mutation changed %d bytes, want 1",
changed,
)
}

if VerifySignature(
publicKey,
mutated,
e13Family2ImmutableSignature,
) {
t.Fatal("V6 mutated payload unexpectedly verified")
}
}