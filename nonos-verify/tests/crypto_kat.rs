// Known-answer tests for the hash the trust chain depends on. BLAKE3 derives
// the NONOS-ID, hashes the capsule payload, and keys the capability-token MAC,
// so a regression here would silently break admission. Vectors are the official
// BLAKE3 reference test vectors.

#[test]
fn blake3_kat_empty() {
    let h = blake3::hash(b"");
    assert_eq!(
        h.to_hex().as_str(),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
}

#[test]
fn blake3_kat_abc() {
    let h = blake3::hash(b"abc");
    assert_eq!(
        h.to_hex().as_str(),
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
    );
}

// Determinism + fixed length: the same input always yields the same 32-byte
// digest. A non-deterministic hash would break the reproducible payload bind.
#[test]
fn blake3_is_deterministic_and_32_bytes() {
    let a = blake3::hash(b"the quick brown fox");
    let b = blake3::hash(b"the quick brown fox");
    assert_eq!(a.as_bytes(), b.as_bytes());
    assert_eq!(a.as_bytes().len(), 32);
}
