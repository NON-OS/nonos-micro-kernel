// Property tests for the trust-chain decoders. The invariant: a malformed
// artifact is an error, never a panic and never a false accept. proptest throws
// thousands of arbitrary byte strings at each decoder.

use nonos_capsule_sign::verify::decode::{
    decode_cert, decode_manifest, decode_trust_anchor_policy,
};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1500))]

    // No arbitrary input may panic any decoder.
    #[test]
    fn decoders_never_panic(data in proptest::collection::vec(any::<u8>(), 0..4096)) {
        let _ = decode_cert(&data);
        let _ = decode_manifest(&data);
        let _ = decode_trust_anchor_policy(&data);
    }

    // Truncating any prefix of a buffer must still never panic (boundary cursor
    // handling is the classic decoder bug).
    #[test]
    fn truncation_never_panics(data in proptest::collection::vec(any::<u8>(), 0..2048), cut in 0usize..2048) {
        let n = cut.min(data.len());
        let _ = decode_cert(&data[..n]);
        let _ = decode_manifest(&data[..n]);
        let _ = decode_trust_anchor_policy(&data[..n]);
    }
}

// A single fixed regression seed: the empty buffer is the smallest malformed
// input and must decode to an error, not a panic.
#[test]
fn empty_buffer_is_rejected_cleanly() {
    assert!(decode_cert(&[]).is_err());
    assert!(decode_manifest(&[]).is_err());
    assert!(decode_trust_anchor_policy(&[]).is_err());
}
