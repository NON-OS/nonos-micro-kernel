// NONOS Operating System (AGPL-3.0-or-later)
//! The bit-exact gate for the all-cores prover. A production emit uses the
//! `parallel` prover to run across every core; this proves it emits the exact
//! same bytes as the serial one. The crate compiles either serial (no feature)
//! or parallel (`--features parallel`) — never both in one binary — so the gate
//! is a two-build diff: this test prints the blake3 of a real preprocessed
//! proof over a small wired AIR, and the harness runs it under both builds and
//! requires the two digests to match. A parallel bug that changed any committed
//! value would change the digest and fail the diff.

use crate::crypto::stark::air::{
    stark_prove_ext_preprocessed, Accumulator, AirExt, RangeCheck, WiredExt,
};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::hash::blake3_hash;
use alloc::boxed::Box;
use alloc::vec::Vec;

fn neg(x: u64) -> Fp {
    Fp::ZERO - Fp::from_u64(x)
}

fn join_split() -> WiredExt {
    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(1, 8);
    WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7))
}

fn witness(air: &WiredExt) -> Vec<Fp> {
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let mut cons = Vec::new();
    let mut acc = Fp::ZERO;
    for &a in &addends {
        cons.push(acc);
        cons.push(a);
        acc = acc + a;
    }
    let mut rng = Vec::new();
    let mut v = 7u64;
    for i in 0..16usize {
        let bit = if i < 15 { v & 1 } else { 0 };
        rng.push(Fp::from_u64(v));
        rng.push(Fp::from_u64(bit));
        if i < 15 {
            v >>= 1;
        }
    }
    air.trace(&[cons, rng])
}

#[test]
fn emit_preprocessed_proof_digest() {
    let air = join_split();
    let w = witness(&air);
    // A raised blowup so the parallel paths (multi-column LDE, wide-leaf
    // hashing, deep-domain commit) are all exercised, not just the trivial one.
    let pre = stark_prove_ext_preprocessed(&air, &w, 32, 8, 3);
    let bytes = crate::production_vector_gen::wire::serialize_pre(&pre);
    let digest = blake3_hash(&bytes);
    let mut hex = alloc::string::String::from("PROOF_DIGEST=");
    for b in &digest {
        hex.push_str(&alloc::format!("{:02x}", b));
    }
    std::println!("{hex} len={}", bytes.len());
}
