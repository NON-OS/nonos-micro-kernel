// NONOS Operating System (AGPL-3.0-or-later)
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::poseidon::permutation::permute;
use crate::crypto::stark::poseidon::sponge::{compress, hash, DIGEST};
use crate::crypto::stark::poseidon::{FULL_ROUNDS, N_ROUNDS, PARTIAL_ROUNDS, WIDTH};

// The real Poseidon-Goldilocks permutation, verified against the published
// reference test vectors. These input/output pairs are the width-12 vectors
// from the Plonky2 test suite, themselves computed with the hadeshash
// reference implementation. Matching them byte for byte is what makes this a
// real hash rather than an invented one: the round constants, the MDS matrix,
// the S-box, and the round schedule must all be exactly the published set or
// a single lane diverges.

const P: u64 = 0xFFFF_FFFF_0000_0001;
const NEG_ONE: u64 = P - 1;

fn state(vals: [u64; WIDTH]) -> [Fp; WIDTH] {
    let mut s = [Fp::ZERO; WIDTH];
    for (i, v) in vals.iter().enumerate() {
        s[i] = Fp::from_u64(*v);
    }
    s
}

fn values(s: [Fp; WIDTH]) -> [u64; WIDTH] {
    let mut out = [0u64; WIDTH];
    for (i, e) in s.iter().enumerate() {
        out[i] = e.value();
    }
    out
}

#[test]
fn the_round_schedule_is_the_published_one() {
    assert_eq!(WIDTH, 12);
    assert_eq!(FULL_ROUNDS, 8);
    assert_eq!(PARTIAL_ROUNDS, 22);
    assert_eq!(N_ROUNDS, 30);
}

#[test]
fn permutation_matches_the_all_zeros_vector() {
    let out = values(permute(state([0; WIDTH])));
    assert_eq!(
        out,
        [
            0x3c18a9786cb0b359,
            0xc4055e3364a246c3,
            0x7953db0ab48808f4,
            0xc71603f33a1144ca,
            0xd7709673896996dc,
            0x46a84e87642f44ed,
            0xd032648251ee0b3c,
            0x1c687363b207df62,
            0xdf8565563e8045fe,
            0x40f5b37ff4254dae,
            0xd070f637b431067c,
            0x1792b1c4342109d7,
        ]
    );
}

#[test]
fn permutation_matches_the_range_vector() {
    let out = values(permute(state([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])));
    assert_eq!(
        out,
        [
            0xd64e1e3efc5b8e9e,
            0x53666633020aaa47,
            0xd40285597c6a8825,
            0x613a4f81e81231d2,
            0x414754bfebd051f0,
            0xcb1f8980294a023f,
            0x6eb2a9e4d54a9d0f,
            0x1902bc3af467e056,
            0xf045d5eafdc6021f,
            0xe4150f77caaa3be5,
            0xc9bfd01d39b50cce,
            0x5c0a27fcb0e1459b,
        ]
    );
}

#[test]
fn permutation_matches_the_all_minus_one_vector() {
    let out = values(permute(state([NEG_ONE; WIDTH])));
    assert_eq!(
        out,
        [
            0xbe0085cfc57a8357,
            0xd95af71847d05c09,
            0xcf55a13d33c1c953,
            0x95803a74f4530e82,
            0xfcd99eb30a135df1,
            0xe095905e913a3029,
            0xde0392461b42919b,
            0x7d3260e24e81d031,
            0x10d3d0465d9deaa0,
            0xa87571083dfc2a47,
            0xe18263681e9958f8,
            0xe28e96f1ae5e60d3,
        ]
    );
}

#[test]
fn permutation_matches_the_random_vector() {
    let input = [
        0x8ccbbbea4fe5d2b7,
        0xc2af59ee9ec49970,
        0x90f7e1a9e658446a,
        0xdcc0630a3ab8b1b8,
        0x7ff8256bca20588c,
        0x5d99a7ca0c44ecfb,
        0x48452b17a70fbee3,
        0xeb09d654690b6c88,
        0x4a55d3a39c676a88,
        0xc0407a38d2285139,
        0xa234bac9356386d1,
        0xe1633f2bad98a52f,
    ];
    let out = values(permute(state(input)));
    assert_eq!(
        out,
        [
            0xa89280105650c4ec,
            0xab542d53860d12ed,
            0x5704148e9ccab94f,
            0xd3a826d4b62da9f5,
            0x8a7a6ca87892574f,
            0xc7017e1cad1a674e,
            0x1f06668922318e34,
            0xa3b203bc8102676f,
            0xfcc781b0ce382bf2,
            0x934c69ff3ed14ba5,
            0x504688a5996e8f13,
            0x401f3f2ed524a2ba,
        ]
    );
}

// The sponge built on the verified permutation: deterministic, length
// separating, and sensitive to every input lane.

#[test]
fn sponge_is_deterministic() {
    let input: [Fp; 5] = [
        Fp::from_u64(11),
        Fp::from_u64(22),
        Fp::from_u64(33),
        Fp::from_u64(44),
        Fp::from_u64(55),
    ];
    assert_eq!(hash(&input), hash(&input));
    assert_eq!(hash(&input).len(), DIGEST);
}

#[test]
fn sponge_separates_lengths_across_the_zero_pad() {
    // A short input and its zero-extension share a padded block but must not
    // collide, because the length is folded into the capacity.
    let short = [Fp::from_u64(7)];
    let padded = [Fp::from_u64(7), Fp::ZERO];
    assert_ne!(hash(&short), hash(&padded));
}

#[test]
fn sponge_depends_on_every_input_element() {
    let base: [Fp; 8] = core::array::from_fn(|i| Fp::from_u64(i as u64 + 1));
    let h0 = hash(&base);
    for i in 0..base.len() {
        let mut m = base;
        m[i] = m[i] + Fp::ONE;
        assert_ne!(hash(&m), h0, "flipping lane {i} did not change the digest");
    }
}

#[test]
fn compress_is_a_two_to_one_map_that_uses_both_sides() {
    let a = [Fp::from_u64(1), Fp::from_u64(2), Fp::from_u64(3), Fp::from_u64(4)];
    let b = [Fp::from_u64(5), Fp::from_u64(6), Fp::from_u64(7), Fp::from_u64(8)];
    assert_eq!(compress(&a, &b), compress(&a, &b));
    assert_ne!(compress(&a, &b), compress(&b, &a), "compress must not be symmetric");
    let mut b2 = b;
    b2[3] = b2[3] + Fp::ONE;
    assert_ne!(compress(&a, &b), compress(&a, &b2), "right input must matter");
}
