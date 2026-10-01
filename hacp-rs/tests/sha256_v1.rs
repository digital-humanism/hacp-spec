use hacp_rs::sha256::sha256_hex;

#[test]
fn sha256_v1_known_answer_vectors() {
    let cases: &[(&[u8], &str)] = &[
        (
            b"",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            b"hacp-sha256-v1-deterministic-verification-input",
            "c82929fd529c3cb002ad6feeacb805998477bf745dcadccbb25b07aad889b03d",
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(sha256_hex(input), *expected);
    }
}

#[test]
fn sha256_v1_lowercase_hex_representation() {
    let digest = sha256_hex(b"abc");

    assert_eq!(digest.len(), 64);
    assert!(digest
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
}
