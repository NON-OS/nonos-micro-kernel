// NONOS Operating System (AGPL-3.0-or-later)
use crate::capabilities::{bit_of, bits, Capability};
use crate::memory::paging::types::permissions::PagePermissions;
use crate::spec;
use crate::usercopy;

// The differential layer of the refinement chain. The Lean theorems prove
// properties of a specification; the Verus proofs show the specified bit
// logic computes the specified result; the tests here run the REAL kernel
// functions, included via #[path], against the same specification restated
// in spec/mod.rs, so the three layers cannot drift apart without a build
// failure. Kani harnesses extend each comparison to all inputs.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

// Capability bit operations: the shipping has/add/remove against the spec,
// through the real Capability enum and its bit assignment.

#[test]
fn capability_bits_match_the_spec_for_every_capability_and_token() {
    for seed in 1..200_000u64 {
        let mut s = seed;
        let token = xorshift(&mut s);
        for cap in Capability::all() {
            let bit = bit_of(cap);
            assert_eq!(bits::has_capability(token, cap), spec::has(token, bit));
            assert_eq!(bits::add_capability(token, cap), spec::grant(token, bit));
            assert_eq!(bits::remove_capability(token, cap), spec::revoke(token, bit));
        }
    }
}

#[test]
fn every_capability_bit_is_a_distinct_single_bit() {
    let all = Capability::all();
    for cap in all {
        let bit = bit_of(cap);
        assert!(bit.is_power_of_two(), "a capability mask must be one bit");
    }
    for (i, a) in all.iter().enumerate() {
        for b in all.iter().skip(i + 1) {
            assert_ne!(bit_of(*a), bit_of(*b), "two capabilities share a bit");
        }
    }
}

#[test]
fn the_bits_caps_conversions_are_inverse_on_the_valid_mask() {
    let valid: u64 = Capability::all().iter().fold(0, |acc, c| acc | bit_of(*c));
    for seed in 1..50_000u64 {
        let mut s = seed;
        let token = xorshift(&mut s);
        let caps = bits::bits_to_caps(token);
        assert_eq!(
            bits::caps_to_bits(&caps),
            token & valid,
            "decoding then encoding must keep exactly the defined bits"
        );
        assert_eq!(bits::capability_count(token & valid), caps.len());
        for cap in &caps {
            assert!(bits::has_capability(token, *cap));
        }
    }
}

// The user-copy range policy: the shipping check_range against the spec,
// exact error variants included.

#[test]
fn check_range_matches_the_spec_on_the_full_edge_set() {
    let interesting_addrs = [
        0u64,
        1,
        0xFFF,
        0x1000,
        spec::USER_SPACE_END - 4096,
        spec::USER_SPACE_END,
        spec::USER_SPACE_END + 1,
        u64::MAX - 4096,
        u64::MAX,
    ];
    let interesting_lens = [
        0usize,
        1,
        0xFFF,
        0x1000,
        spec::MAX_COPY_SIZE - 1,
        spec::MAX_COPY_SIZE,
        spec::MAX_COPY_SIZE + 1,
        usize::MAX,
    ];
    for &addr in &interesting_addrs {
        for &len in &interesting_lens {
            assert_eq!(usercopy::check(addr, len), spec::check_range(addr, len));
        }
    }
    for seed in 1..200_000u64 {
        let mut s = seed;
        let addr = xorshift(&mut s);
        let len = xorshift(&mut s) as usize;
        assert_eq!(usercopy::check(addr, len), spec::check_range(addr, len));
    }
}

// The page-permission encoding: the shipping to_pte_flags and
// is_wx_violation against the spec, every permission bit pattern sampled.

#[test]
fn pte_encoding_matches_the_spec() {
    for seed in 1..200_000u64 {
        let mut s = seed;
        let perm = xorshift(&mut s) as u32;
        let p = PagePermissions::from_bits(perm);
        assert_eq!(p.to_pte_flags(), spec::pte_flags(perm));
        assert_eq!(p.is_wx_violation(), spec::wx_violation(perm));
    }
}
