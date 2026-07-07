// NONOS Operating System (AGPL-3.0-or-later)
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::transcript::Transcript;

extern crate alloc;
use alloc::vec::Vec;

// The Fiat-Shamir transcript turns the interactive protocol into a
// non-interactive proof: the challenges are derived from the committed data,
// so a prover cannot choose them and cannot reuse a proof for a different
// statement. These checks exercise the properties the soundness of that
// transformation rests on: the challenge stream is a deterministic function
// of the absorbed sequence, it changes under any change to that sequence, and
// the query indices stay in range.

fn draw_challenges(label: &[u8], absorb: &[[u8; 32]]) -> (Vec<u64>, Vec<usize>) {
    let mut t = Transcript::new(label);
    for d in absorb {
        t.absorb_digest(d);
    }
    let fps: Vec<u64> = (0..8).map(|_| t.challenge_fp().value()).collect();
    let idxs: Vec<usize> = (0..8).map(|_| t.challenge_index(64)).collect();
    (fps, idxs)
}

fn digest(byte: u8) -> [u8; 32] {
    [byte; 32]
}

#[test]
fn the_challenge_stream_is_deterministic() {
    let absorb = [digest(1), digest(2), digest(3)];
    let a = draw_challenges(b"NONOS-STARK", &absorb);
    let b = draw_challenges(b"NONOS-STARK", &absorb);
    assert_eq!(a, b, "the same transcript drew different challenges");
}

#[test]
fn a_different_label_gives_a_different_stream() {
    let absorb = [digest(1)];
    let a = draw_challenges(b"label-one", &absorb);
    let b = draw_challenges(b"label-two", &absorb);
    assert_ne!(a.0, b.0, "distinct labels shared a challenge stream");
}

#[test]
fn a_one_bit_change_in_absorbed_data_changes_every_later_challenge() {
    // This is the Fiat-Shamir binding: flipping a single bit of any committed
    // value re-randomizes the challenges, so a proof cannot be transplanted to
    // a different statement.
    let base = [digest(0x10), digest(0x20), digest(0x30)];
    let (base_fp, base_idx) = draw_challenges(b"NONOS-STARK", &base);
    for pos in 0..base.len() {
        for bit in 0..8u32 {
            let mut m = base;
            m[pos][0] ^= 1 << bit;
            let (fp, idx) = draw_challenges(b"NONOS-STARK", &m);
            assert_ne!(fp, base_fp, "a flipped bit at {pos}/{bit} left the fp stream unchanged");
            assert_ne!(idx, base_idx, "a flipped bit at {pos}/{bit} left the index stream unchanged");
        }
    }
}

#[test]
fn absorb_order_matters() {
    // The state folds each input into a running hash, so a permutation of the
    // absorbed sequence yields a different stream.
    let ab = draw_challenges(b"NONOS-STARK", &[digest(1), digest(2)]);
    let ba = draw_challenges(b"NONOS-STARK", &[digest(2), digest(1)]);
    assert_ne!(ab.0, ba.0, "reordering the absorbed data left the stream unchanged");
}

#[test]
fn digest_and_field_absorbs_are_domain_separated() {
    // A 32-byte digest and a field element that share leading bytes must not
    // collide, because each absorb carries a distinct domain tag.
    let mut td = Transcript::new(b"NONOS-STARK");
    td.absorb_digest(&{
        let mut d = [0u8; 32];
        d[0..8].copy_from_slice(&7u64.to_le_bytes());
        d
    });
    let cd = td.challenge_fp().value();

    let mut tf = Transcript::new(b"NONOS-STARK");
    tf.absorb_fp(Fp::from_u64(7));
    let cf = tf.challenge_fp().value();

    assert_ne!(cd, cf, "a digest and a field element were not domain separated");
}

#[test]
fn consecutive_challenges_advance() {
    // Each challenge mixes a tag into the state, so the stream does not repeat
    // a value back to back.
    let mut t = Transcript::new(b"NONOS-STARK");
    t.absorb_digest(&digest(9));
    let first = t.challenge_fp().value();
    let second = t.challenge_fp().value();
    assert_ne!(first, second, "two consecutive challenges were equal");
}

#[test]
fn query_indices_stay_within_every_power_of_two_bound() {
    for log_bound in 1..=16u32 {
        let bound = 1usize << log_bound;
        let mut t = Transcript::new(b"NONOS-STARK");
        t.absorb_digest(&digest(log_bound as u8));
        for _ in 0..2000 {
            let i = t.challenge_index(bound);
            assert!(i < bound, "index {i} out of bound {bound}");
        }
    }
}

#[test]
fn query_indices_span_their_range() {
    // Over enough draws the masked index reaches both ends of a small domain,
    // evidence that the low bits are not stuck.
    let bound = 8usize;
    let mut t = Transcript::new(b"NONOS-STARK");
    t.absorb_digest(&digest(42));
    let mut seen = [false; 8];
    for _ in 0..1000 {
        seen[t.challenge_index(bound)] = true;
    }
    assert!(seen.iter().all(|&b| b), "some query index in 0..8 never appeared");
}
