// NONOS Operating System (AGPL-3.0-or-later)
//! The inner join-split fixture the assembly verifies: the wired Accumulator +
//! RangeCheck stand-in, proven Poseidon-committed while absorbing the intent
//! publics, plus the replayed composition inputs every region reads from.

use crate::crypto::stark::air::{
    compose_inputs_pub, stark_prove_poseidon_ext_pub, Accumulator, Air, AirExt, ComposeInputs,
    Poseidon, RangeCheck, StarkProofExtP, WiredExt, RATE,
};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) const NQ: usize = 32;
pub(crate) const GRIND: u32 = 16;
pub(crate) const EXTRA: u32 = 3;

pub(crate) fn hasher() -> Poseidon {
    Poseidon::new(2, [Fp::ZERO; RATE])
}

pub(crate) struct Inner {
    pub air: WiredExt,
    pub publics: Vec<Fp>,
    pub proof: StarkProofExtP,
    pub ci: ComposeInputs,
    pub t: u64,
    pub g: Fp,
}

pub(crate) fn join_split(h: &Poseidon) -> Inner {
    let (words, k_intents) = (11usize, 2usize);
    let mut publics = Vec::with_capacity(k_intents * words);
    for i in 0..k_intents * words {
        publics.push(Fp::from_u64(0xA000 + i as u64));
    }

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sig: Vec<usize> = (0..32).collect();
    sig.swap(1, 8);
    let air = WiredExt::new(regions, alloc::vec![0], sig, Fp::from_u64(5), Fp::from_u64(7));

    let neg = |x: u64| Fp::ZERO - Fp::from_u64(x);
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

    let witness = air.trace(&[cons, rng]);
    let proof = stark_prove_poseidon_ext_pub(&air, &witness, NQ, GRIND, EXTRA, h, &publics);
    let ci = compose_inputs_pub(&air, &proof, EXTRA, h, &publics);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());
    Inner { air, publics, proof, ci, t, g }
}
