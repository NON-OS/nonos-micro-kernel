// NONOS Operating System (AGPL-3.0-or-later)
//! The Poseidon-committed money-grade FRI, checked against its spec: a low-degree
//! extension codeword verifies, a random one is rejected, and the challenges are
//! extension-field. This is the inner form recursion folds over.

use crate::crypto::stark::air::Poseidon;
use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::fri_poseidon_ext::{fri_prove_poseidon_ext, fri_verify_poseidon_ext};
use crate::crypto::stark::poly::eval;

extern crate alloc;
use alloc::vec::Vec;

const RATE: usize = 4;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// A low-degree extension codeword: a base polynomial of degree `< d` evaluated on
/// the coset, lifted into `Fp2`.
fn low_degree_ext(log_n: u32, d: usize, shift: Fp, seed: u64) -> Vec<Fp2> {
    let n = 1usize << log_n;
    let omega = root_of_unity(log_n);
    let mut s = seed | 1;
    let coeffs: Vec<Fp> = (0..d).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    let mut x = shift;
    let mut cw = Vec::with_capacity(n);
    for _ in 0..n {
        cw.push(Fp2::from_base(eval(&coeffs, x)));
        x = x * omega;
    }
    cw
}

fn hasher() -> Poseidon {
    Poseidon::new(2, [Fp::ZERO; RATE])
}

fn squaring_trace(log_t: u32, seed: Fp) -> Vec<Fp> {
    let t = 1usize << log_t;
    let mut trace = Vec::with_capacity(t);
    let mut cur = seed;
    for _ in 0..t {
        trace.push(cur);
        cur = cur * cur;
    }
    trace
}

#[test]
fn a_poseidon_committed_stark_proves_and_verifies() {
    use crate::crypto::stark::air::{
        stark_prove_poseidon_ext, stark_verify_poseidon_ext, Squaring,
    };
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 4, seed };
    let trace = squaring_trace(4, seed);
    let h = hasher();
    let proof = stark_prove_poseidon_ext(&air, &trace, 32, 8, 0, &h);
    assert!(
        stark_verify_poseidon_ext(&air, &proof, 32, 8, 0, &h),
        "an honest Poseidon-committed STARK was rejected"
    );
}

#[test]
fn a_tampered_poseidon_committed_stark_is_rejected() {
    use crate::crypto::stark::air::{
        stark_prove_poseidon_ext, stark_verify_poseidon_ext, Squaring,
    };
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 4, seed };
    let mut trace = squaring_trace(4, seed);
    trace[2] = trace[2] + Fp::from_u64(1); // break the squaring relation
    let h = hasher();
    let proof = stark_prove_poseidon_ext(&air, &trace, 32, 8, 0, &h);
    assert!(
        !stark_verify_poseidon_ext(&air, &proof, 32, 8, 0, &h),
        "a tampered Poseidon-committed STARK verified"
    );
}

#[test]
fn a_poseidon_committed_join_split_core_proves_and_verifies() {
    // The real inner proof recursion folds over: the wired conservation + range
    // join-split core, committed with Poseidon at deployment soundness (rate 1/16).
    use crate::crypto::stark::air::{
        stark_prove_poseidon_ext, stark_verify_poseidon_ext, Accumulator, AirExt, RangeCheck,
        WiredExt,
    };
    use alloc::boxed::Box;

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(1, 8); // conservation acc[1] (=input 7) wired to range acc[0]
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));

    let neg = |x: u64| -> Fp { Fp::ZERO - Fp::from_u64(x) };
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let mut cons = Vec::with_capacity(addends.len() * 2);
    let mut acc = Fp::ZERO;
    for &a in &addends {
        cons.push(acc);
        cons.push(a);
        acc = acc + a;
    }
    let mut rng = Vec::with_capacity(32);
    let mut v = 7u64;
    for i in 0..16usize {
        let bit = if i < 15 { v & 1 } else { 0 };
        rng.push(Fp::from_u64(v));
        rng.push(Fp::from_u64(bit));
        if i < 15 {
            v >>= 1;
        }
    }
    let witness = wired.trace(&[cons, rng]);
    let h = hasher();
    let proof = stark_prove_poseidon_ext(&wired, &witness, 32, 16, 3, &h);
    assert!(
        stark_verify_poseidon_ext(&wired, &proof, 32, 16, 3, &h),
        "the Poseidon-committed join-split core was rejected"
    );
}

// Build the real Poseidon-committed join-split core proof recursion folds over,
// returning the AIR alongside so its verification witness can be extracted.
fn poseidon_join_split_proof(
    h: &Poseidon,
    nq: usize,
    grind: u32,
    extra: u32,
) -> (crate::crypto::stark::air::WiredExt, crate::crypto::stark::air::StarkProofExtP) {
    use crate::crypto::stark::air::{
        stark_prove_poseidon_ext, Accumulator, AirExt, RangeCheck, WiredExt,
    };
    use alloc::boxed::Box;
    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(1, 8);
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let neg = |x: u64| -> Fp { Fp::ZERO - Fp::from_u64(x) };
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
    let witness = wired.trace(&[cons, rng]);
    let proof = stark_prove_poseidon_ext(&wired, &witness, nq, grind, extra, h);
    (wired, proof)
}

// The first real recursion fragment over the actual inner proof: take the real
// Poseidon join-split proof's FRI, replay its transcript to recover the Fp2 fold
// challenges, then prove IN-CIRCUIT (a STARK) that its query-0 fold chain is
// consistent. This is verification of the real proof's low-degree test, arithmetized.
#[test]
fn the_real_poseidon_fri_fold_chain_verifies_in_circuit() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, TraceFoldExt};
    use crate::crypto::stark::field::Fp2;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (_air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    // Replay the FRI transcript to recover the fold challenges and the first index.
    let mut ts = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    for root in &fri.roots {
        ts.absorb_digest(root);
        betas.push(ts.challenge_fp2());
    }
    for value in &fri.final_layer {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    assert!(ts.verify_pow(fri.pow_nonce, grind), "P's FRI proof-of-work did not check");
    let q0 = ts.challenge_index(n);

    // Extract query 0's real openings and the public domain data per layer.
    let final_value = fri.final_layer[0];
    let base_omega = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let layers = &fri.queries[0].layers;
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let (mut x_inv, mut dir) = (Vec::new(), Vec::new());
    for (m, op) in layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let half = n >> (m + 1);
        let i = q0 % half;
        let x = (shift * base_omega.pow(i as u64)).pow(1u64 << m);
        x_inv.push(x.inv());
        let half_next = n >> (m + 2);
        dir.push(i >= half_next);
    }
    a.push(final_value);
    b.push(final_value);

    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new(log_layers, n_folds, x_inv, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);
    let fproof = stark_prove_ext(&fold, &ftrace, 32, 8);
    assert!(
        stark_verify_ext(&fold, &fproof, 32, 8),
        "the real Poseidon join-split proof's FRI fold chain was rejected in-circuit"
    );
}

// The second real recursion fragment: take the real Poseidon join-split proof's
// FRI layer-0 opening and prove IN-CIRCUIT (a STARK) that its Poseidon Merkle path
// authenticates against the committed root. This is verification of the real
// proof's commitment openings, arithmetized.
#[test]
fn a_real_poseidon_merkle_opening_verifies_in_circuit() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, MultiMembership, Opening};
    use crate::crypto::stark::poseidon_merkle::pack_ext;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (_air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    // Replay to the first query index.
    let mut ts = PoseidonTranscript::new(h.clone());
    for root in &fri.roots {
        ts.absorb_digest(root);
        ts.challenge_fp2();
    }
    for value in &fri.final_layer {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    assert!(ts.verify_pow(fri.pow_nonce, grind));
    let q0 = ts.challenge_index(n);

    // Layer 0 opens position i = q0 % (n/2) against roots[0].
    let i = q0 % (n >> 1);
    let op = &fri.queries[0].layers[0];
    let siblings = op.a_path.clone();
    let depth = siblings.len();
    let directions: Vec<bool> = (0..depth).map(|l| (i >> l) & 1 == 1).collect();
    let opening = Opening { leaf: pack_ext(op.a), root: fri.roots[0], siblings, directions };
    let mem = MultiMembership::new(h.clone(), 2, alloc::vec![opening]);
    let mtrace = mem.trace();
    let mproof = stark_prove_ext(&mem, &mtrace, 32, 8);
    assert!(
        stark_verify_ext(&mem, &mproof, 32, 8),
        "the real Poseidon join-split proof's Merkle opening was rejected in-circuit"
    );
}

// The production form of the Merkle region: each compression's direction (boolean
// constrained) and sibling ride the trace, so the AIR is instance-independent
// (round constants, the slot and opening selectors, and the reset column are the
// only periodic columns, nothing pinned). The opened leaf and the checkpoint root
// become witness, bound by the assembly to the fold and the transcript. It
// authenticates the same real opening.
#[test]
fn the_merkle_witness_form_authenticates_the_real_opening() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Air, MultiMembership, Opening, RATE, WIDTH,
    };
    use crate::crypto::stark::poseidon_merkle::pack_ext;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (_air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    let mut ts = PoseidonTranscript::new(h.clone());
    for root in &fri.roots {
        ts.absorb_digest(root);
        ts.challenge_fp2();
    }
    for value in &fri.final_layer {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    assert!(ts.verify_pow(fri.pow_nonce, grind));
    let q0 = ts.challenge_index(n);

    let i = q0 % (n >> 1);
    let op = &fri.queries[0].layers[0];
    let siblings = op.a_path.clone();
    let depth = siblings.len();
    let directions: Vec<bool> = (0..depth).map(|l| (i >> l) & 1 == 1).collect();
    let opening = Opening { leaf: pack_ext(op.a), root: fri.roots[0], siblings, directions };
    let mem = MultiMembership::new_witness(h.clone(), 2, alloc::vec![opening]);
    // Instance-independent AIR: direction plus RATE sibling columns in the trace,
    // no pinned boundary.
    assert_eq!(mem.trace_width(), WIDTH + 1 + RATE);
    assert_eq!(mem.boundary().len(), 0);
    let mtrace = mem.trace();
    let mproof = stark_prove_ext(&mem, &mtrace, 32, 8);
    assert!(
        stark_verify_ext(&mem, &mproof, 32, 8),
        "the production-form Merkle opening was rejected in-circuit"
    );
}

// The authentication the recursion was missing: the DEEP consistency uses the
// opened DEEP value, the composition, and every trace column, and a sound verifier
// authenticates all three against their commitments (deep against the FRI root,
// comp against the composition root, each trace column against its trace root),
// exactly as the inner verifier does. This proves the whole opening set of the real
// proof authenticates in-circuit as one batched membership region, so the values
// feeding the DEEP check are committed, not trusted.
#[test]
fn the_full_query_opening_set_authenticates_in_circuit() {
    use crate::crypto::stark::air::{
        query_openings_query0, stark_prove_ext, stark_verify_ext, Air, MultiMembership,
    };
    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let openings = query_openings_query0(&air, &proof, extra, &h, &[]);
    // The DEEP value, the composition, and every one of the inner trace columns.
    assert_eq!(openings.len(), air.trace_width() + 2);
    let mem = MultiMembership::new_witness(h.clone(), 2, openings);
    let mtrace = mem.trace();
    let mproof = stark_prove_ext(&mem, &mtrace, 32, 8);
    assert!(
        stark_verify_ext(&mem, &mproof, 32, 8),
        "the batched query-opening authentication was rejected in-circuit"
    );
}

// Native validation for the DEEP-x derivation: the query evaluation point x =
// shift * omega^p is not a copy of any cell, so it must be derived in-circuit from
// the consistency index p (whose bits are the deep-opening directions) as the
// product chain shift * prod_k (omega^(2^k))^(bit_k). This proves the formula and
// the bit source reproduce the real x before any constraint is written.
#[test]
fn the_deep_x_product_chain_matches_native() {
    use crate::crypto::stark::air::{deep_terms_query0, query_openings_query0};
    use crate::crypto::stark::fri::root_of_unity;
    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let (_terms, dx, _ddeep) = deep_terms_query0(&air, &proof, extra, &h);

    // The bits of p, LSB first, are the deep opening's path directions.
    let ops = query_openings_query0(&air, &proof, extra, &h, &[]);
    let dirs = &ops[1].directions;
    let p: usize = dirs.iter().enumerate().map(|(lv, &b)| (b as usize) << lv).sum();

    let n_folds = proof.fri.roots.len();
    let blowup = proof.fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(7);

    let mut x = Fp2::from_base(shift);
    for k in 0..log_n {
        if (p >> k) & 1 == 1 {
            x = x * Fp2::from_base(omega.pow(1u64 << k));
        }
    }
    assert_eq!(x, dx, "the product chain does not reproduce the real DEEP x");
}

// The DEEP-x derivation as an in-circuit region: prove the running product computes
// shift * omega^p from the index bits, and its final point equals the real DEEP x.
// The bits and the point are witness (bound by the assembly); only shift is pinned.
#[test]
fn the_index_point_region_derives_the_real_deep_x() {
    use crate::crypto::stark::air::{
        deep_terms_query0, query_openings_query0, stark_prove_ext, stark_verify_ext, Air,
        IndexPoint,
    };
    use crate::crypto::stark::fri::root_of_unity;
    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let (_terms, dx, _ddeep) = deep_terms_query0(&air, &proof, extra, &h);

    let ops = query_openings_query0(&air, &proof, extra, &h, &[]);
    let dirs = &ops[1].directions;
    let p: usize = dirs.iter().enumerate().map(|(lv, &b)| (b as usize) << lv).sum();
    let bits = dirs.len();

    let n_folds = proof.fri.roots.len();
    let blowup = proof.fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(7);

    let ip = IndexPoint::new(omega, shift, bits, p);
    assert_eq!(ip.point(), dx, "the region's derived point is not the real DEEP x");
    let tr = ip.trace();
    let iproof = stark_prove_ext(&ip, &tr, 32, 8);
    assert!(
        stark_verify_ext(&ip, &iproof, 32, 8),
        "the index-point derivation was rejected in-circuit"
    );
}

// The third real recursion fragment: verify the real Poseidon join-split proof's
// DEEP consistency for query 0 in-circuit -- every opened column against its
// out-of-domain claim, plus the composition against its claim, batched to the
// query's DEEP value. This is verification of the real proof's DEEP quotient,
// arithmetized.
#[test]
fn the_real_poseidon_deep_consistency_verifies_in_circuit() {
    use crate::crypto::stark::air::{
        deep_terms_query0, stark_prove_ext, stark_verify_ext, DeepCheckExt,
    };
    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let (terms, x, deep) = deep_terms_query0(&air, &proof, extra, &h);
    let dc = DeepCheckExt::new(terms, x, deep);
    let dtrace = dc.trace();
    let dproof = stark_prove_ext(&dc, &dtrace, 32, 8);
    assert!(
        stark_verify_ext(&dc, &dproof, 32, 8),
        "the real Poseidon join-split proof's DEEP consistency was rejected in-circuit"
    );
}

// The production form of the DEEP region: the per-term data (val, claim, point,
// coeff) and the evaluation point x ride the trace, not periodic columns, so the
// AIR is instance-independent (the term and composition selectors are the only
// periodic columns, acc-starts-zero the only boundary). x is constrained constant
// across terms; the terms and the final DEEP value become witness, bound by the
// assembly grand product. It proves the same real DEEP consistency.
#[test]
fn the_deep_witness_form_verifies_the_real_consistency() {
    use crate::crypto::stark::air::{
        deep_terms_query0, stark_prove_ext, stark_verify_ext, Air, DeepCheckExt,
    };
    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let (terms, x, deep) = deep_terms_query0(&air, &proof, extra, &h);
    let dc = DeepCheckExt::new_witness(terms, x, deep);
    // Instance-independent AIR: 16 trace columns, 3 structural periodic (two
    // selectors and the g^k schedule), 2 boundaries.
    assert_eq!(dc.trace_width(), 16);
    assert_eq!(dc.periodic_columns().len(), 3);
    assert_eq!(dc.boundary().len(), 2);
    let dtrace = dc.trace();
    let dproof = stark_prove_ext(&dc, &dtrace, 32, 8);
    assert!(
        stark_verify_ext(&dc, &dproof, 32, 8),
        "the production-form DEEP consistency was rejected in-circuit"
    );
}

// The inlined compose_ext formula for the join-split, validated natively against
// the real compose_ext: this pins the arithmetic the compose-at-z AIR must encode
// (transition values out0..out2, the exempt/vanishing factor, boundary quotients)
// before it is committed to constraints.
#[test]
fn the_join_split_compose_formula_matches_compose_ext() {
    use crate::crypto::stark::air::{compose_inputs, Air};
    use crate::crypto::stark::field::Fp2;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);

    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());
    let w = &proof.ood_frame;
    let (w0, w1, w2, w3, w5) = (w[0], w[1], w[2], w[3], w[5]);
    let p = &ci.periodic_z;
    let (sel0, sel1, id, sig, gp_sel) = (p[0], p[1], p[2], p[3], p[4]);
    let beta = Fp2::from_base(Fp::from_u64(5));
    let gamma = Fp2::from_base(Fp::from_u64(7));
    let two = Fp2::from_base(Fp::from_u64(2));

    let out0 = sel0 * (w3 - w0 - w1) + sel1 * (w0 - two * w3 - w1);
    let out1 = sel1 * (w1 * (w1 - Fp2::ONE));
    let num = w0 + beta * id + gamma;
    let den = w0 + beta * sig + gamma;
    let out2 = gp_sel * (w5 * den - w2 * num) + (Fp2::ONE - gp_sel) * (w5 - w2);

    let z = ci.z;
    let z_h_inv = (z.pow(t) - Fp2::ONE).inv();
    let exempt = z - Fp2::from_base(g.pow(t - 1));
    let e = exempt * z_h_inv;

    let mut acc = ci.coeffs[0] * out0 * e + ci.coeffs[1] * out1 * e + ci.coeffs[2] * out2 * e;
    for (j, (col, row, expected)) in air.boundary().iter().enumerate() {
        let q =
            (w[*col] - Fp2::from_base(*expected)) * (z - Fp2::from_base(g.pow(*row as u64))).inv();
        acc = acc + ci.coeffs[3 + j] * q;
    }
    assert_eq!(acc, ci.comp_z, "the inlined join-split compose formula did not match compose_ext");
}

// The fourth and hardest recursion fragment: verify compose_ext AT z in-circuit
// over the real proof -- the meta-circular piece that re-derives the composition
// value the DEEP check consumes from the out-of-domain frame, arithmetizing the
// join-split's own transition_ext plus the vanishing and boundary quotients. With
// this the composition value is no longer trusted; it is proven.
#[test]
fn the_real_poseidon_compose_at_z_verifies_in_circuit() {
    use crate::crypto::stark::air::{
        compose_inputs, stark_prove_ext, stark_verify_ext, Air, ComposeBoundary, ComposeCheck,
    };
    use crate::crypto::stark::field::Fp2;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut periodic = [Fp2::ZERO; 5];
    periodic.copy_from_slice(&ci.periodic_z[..5]);
    let mut coeffs = [Fp2::ZERO; 8];
    coeffs.copy_from_slice(&ci.coeffs[..8]);
    let boundaries: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, expected)| ComposeBoundary {
            col: *col,
            g_row: g.pow(*row as u64),
            expected: *expected,
        })
        .collect();

    let cc =
        ComposeCheck::new(window, periodic, coeffs, ci.z, ci.comp_z, g.pow(t - 1), t, boundaries);
    let ctrace = cc.trace();
    let cproof = stark_prove_ext(&cc, &ctrace, 32, 8);
    assert!(
        stark_verify_ext(&cc, &cproof, 32, 8),
        "the real proof's compose_ext at z was rejected in-circuit"
    );
}

// The compose-at-z check must reject a composition value that is not the honest
// combination of the frame: a prover cannot substitute a convenient comp_z.
#[test]
fn the_real_poseidon_compose_at_z_rejects_a_wrong_value() {
    use crate::crypto::stark::air::{
        compose_inputs, stark_prove_ext, stark_verify_ext, Air, ComposeBoundary, ComposeCheck,
    };
    use crate::crypto::stark::field::Fp2;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut periodic = [Fp2::ZERO; 5];
    periodic.copy_from_slice(&ci.periodic_z[..5]);
    let mut coeffs = [Fp2::ZERO; 8];
    coeffs.copy_from_slice(&ci.coeffs[..8]);
    let boundaries: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, expected)| ComposeBoundary {
            col: *col,
            g_row: g.pow(*row as u64),
            expected: *expected,
        })
        .collect();

    let wrong = ci.comp_z + Fp2::from_base(Fp::from_u64(1));
    let cc = ComposeCheck::new(window, periodic, coeffs, ci.z, wrong, g.pow(t - 1), t, boundaries);
    let ctrace = cc.trace();
    let cproof = stark_prove_ext(&cc, &ctrace, 32, 8);
    assert!(!stark_verify_ext(&cc, &cproof, 32, 8), "a dishonest composition value verified");
}

// Pin the exact sponge alignment before arithmetizing it: a hand-run Poseidon
// sponge (absorb = inject into lane 0 then permute; squeeze = read lane 0 then
// permute) must reproduce the real proof's STARK challenges bit for bit. This is
// the ground truth the transcript-derivation AIR must match.
#[test]
fn the_transcript_sponge_reproduces_the_stark_challenges() {
    use crate::crypto::stark::air::{compose_inputs, WIDTH};
    use crate::crypto::stark::field::Fp2;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);

    let mut st = [Fp::ZERO; WIDTH];
    let absorb = |st: &mut [Fp; WIDTH], v: Fp| {
        st[0] = st[0] + v;
        *st = h.permute(*st);
    };
    let squeeze = |st: &mut [Fp; WIDTH]| -> Fp {
        let c = st[0];
        *st = h.permute(*st);
        c
    };

    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut st, *lane);
        }
    }
    let ncoeffs = ci.coeffs.len();
    let mut coeffs = Vec::with_capacity(ncoeffs);
    for _ in 0..ncoeffs {
        let c0 = squeeze(&mut st);
        let c1 = squeeze(&mut st);
        coeffs.push(Fp2::new(c0, c1));
    }
    assert_eq!(coeffs, ci.coeffs, "the hand-run sponge did not reproduce the coefficients");

    for lane in &proof.comp_root {
        absorb(&mut st, *lane);
    }
    let z0 = squeeze(&mut st);
    let z1 = squeeze(&mut st);
    assert_eq!(
        Fp2::new(z0, z1),
        ci.z,
        "the hand-run sponge did not reproduce the out-of-domain point"
    );
}

// The fifth recursion fragment: prove the real proof's Fiat-Shamir challenges were
// honestly squeezed from its committed data, in-circuit. The absorbed sequence is
// the proof's (trace roots, composition root, out-of-domain frame); the squeezed
// coefficients, out-of-domain point, and DEEP coefficients are pinned. With this
// the challenges are proven, not trusted.
#[test]
fn the_real_poseidon_transcript_derivation_verifies_in_circuit() {
    use crate::crypto::stark::air::{
        compose_inputs, stark_prove_ext, stark_verify_ext, Air, TranscriptCheck, TranscriptOp,
        WIDTH,
    };

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let width = air.trace_width();
    let window = air.window_size();

    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], v: Fp| {
        ops.push(TranscriptOp::Absorb(v));
        st[0] = st[0] + v;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };

    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    for _ in 0..2 {
        squeeze(&mut ops, &mut st);
    }
    for v in &proof.ood_frame {
        absorb(&mut ops, &mut st, v.c0);
        absorb(&mut ops, &mut st, v.c1);
    }
    for _ in 0..(width * window + 1) * 2 {
        squeeze(&mut ops, &mut st);
    }

    let tc = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = tc.trace();
    let tproof = stark_prove_ext(&tc, &ttrace, 32, 8);
    assert!(
        stark_verify_ext(&tc, &tproof, 32, 8),
        "the real proof's transcript derivation was rejected in-circuit"
    );
}

// The production form of the transcript region: the absorbed value rides the trace,
// gated by a structural inject selector, and no squeeze is pinned. The AIR is then
// instance-independent (round constants and the selector are the only periodic
// columns, sponge-empty the only boundaries), which is what a fixed on-chain
// verifier needs. The absorbed values and squeezed challenges become witness, bound
// by the assembly grand product to their sources and consumers.
#[test]
fn the_transcript_witness_form_proves_the_same_sponge_without_pinning() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Air, TranscriptCheck, TranscriptOp, WIDTH,
    };
    let h = hasher();
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], val: Fp| {
        ops.push(TranscriptOp::Absorb(val));
        st[0] = st[0] + val;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for i in 0..6u64 {
        absorb(&mut ops, &mut st, Fp::from_u64(7 * i + 1));
    }
    for _ in 0..4 {
        squeeze(&mut ops, &mut st);
    }

    let tc = TranscriptCheck::new_witness(h.clone(), 2, ops);
    // One extra trace column for the absorbed value; the AIR is instance-independent.
    assert_eq!(tc.trace_width(), WIDTH + 1);
    assert_eq!(tc.periodic_columns().len(), WIDTH + 1);
    assert_eq!(tc.boundary().len(), WIDTH, "only the sponge-empty boundaries remain");
    let tr = tc.trace();
    // Native check first: every transition row must vanish, boundaries must hold.
    let w = tc.trace_width();
    let n = 1usize << tc.log_trace_len();
    let per = tc.periodic_columns();
    for row in 0..n - 1 {
        let window: Vec<Fp> = tr[row * w..(row + 2) * w].to_vec();
        let pr: Vec<Fp> = per.iter().map(|c| c[row]).collect();
        let out = tc.transition(&window, &pr);
        assert!(
            out.iter().all(|v| *v == Fp::ZERO),
            "witness transition nonzero at row {}: {:?}",
            row,
            out
        );
    }
    let proof = stark_prove_ext(&tc, &tr, 32, 8);
    assert!(
        stark_verify_ext(&tc, &proof, 32, 8),
        "the production-form transcript sponge did not verify"
    );
}

// The assembly begins: wire the transcript-derivation region and the compose-at-z
// region into ONE proof, binding the squeezed out-of-domain point to the point the
// composition is evaluated at. So compose no longer trusts z; it is the z the
// transcript proved was squeezed. The grand product over the shared cells is the
// copy constraint.
#[ignore]
#[test]
fn the_transcript_and_compose_are_wired_into_one_proof() {
    use crate::crypto::stark::air::{
        compose_inputs, stark_prove_ext, stark_verify_ext, Air, AirExt, ComposeBoundary,
        ComposeCheck, TranscriptCheck, TranscriptOp, WiredExt, WIDTH,
    };
    use crate::crypto::stark::field::Fp2;
    use alloc::boxed::Box;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    // Region 1: compose-at-z.
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut periodic = [Fp2::ZERO; 5];
    periodic.copy_from_slice(&ci.periodic_z[..5]);
    let mut coeffs = [Fp2::ZERO; 8];
    coeffs.copy_from_slice(&ci.coeffs[..8]);
    let boundaries: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, expected)| ComposeBoundary {
            col: *col,
            g_row: g.pow(*row as u64),
            expected: *expected,
        })
        .collect();
    let compose =
        ComposeCheck::new(window, periodic, coeffs, ci.z, ci.comp_z, g.pow(t - 1), t, boundaries);
    let ctrace = compose.trace();

    // Region 0: transcript derivation. z is squeezed at operations after the trace
    // roots, the coefficients, and the composition root.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], v: Fp| {
        ops.push(TranscriptOp::Absorb(v));
        st[0] = st[0] + v;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    let z_op = ops.len(); // the operation index where z.c0 is squeezed
    squeeze(&mut ops, &mut st);
    squeeze(&mut ops, &mut st);
    let transcript = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    let regions: Vec<Box<dyn AirExt>> =
        alloc::vec![Box::new(transcript) as Box<dyn AirExt>, Box::new(compose)];
    let l = 4usize; // permutation rounds
    let t_height = 1usize << regions[0].log_trace_len();
    let span = (t_height + (1usize << regions[1].log_trace_len())).next_power_of_two();

    // wired columns: the transcript squeeze lane (0) and compose's z (22, 23) and
    // coefficient cells (24..39).
    let mut wired_cols = alloc::vec![0usize];
    for c in 22..40 {
        wired_cols.push(c);
    }
    let k = wired_cols.len();
    let widx = |col: usize| -> usize { wired_cols.iter().position(|&c| c == col).unwrap() };
    let mut sigma: Vec<usize> = (0..span * k).collect();
    let c_row = t_height; // compose region row 0
                          // z: transcript operations z_op, z_op+1 wire to compose columns 22, 23.
    sigma.swap((z_op * l) * k, c_row * k + widx(22));
    sigma.swap(((z_op + 1) * l) * k, c_row * k + widx(23));
    // The 8 coefficients: transcript operations 12+2i, 12+2i+1 (after the 12 root
    // absorbs) wire to compose columns 24+2i, 25+2i.
    for i in 0..8 {
        sigma.swap(((12 + 2 * i) * l) * k, c_row * k + widx(24 + 2 * i));
        sigma.swap(((12 + 2 * i + 1) * l) * k, c_row * k + widx(25 + 2 * i));
    }

    let wired = WiredExt::new(regions, wired_cols, sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[ttrace, ctrace]);
    let wproof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &wproof, 32, 8),
        "the transcript and compose regions were not consistently wired on the challenges"
    );
}

// Three regions in one proof: the transcript, the composition, and the DEEP check,
// with the composition value compose proved bound to the value DEEP consumes (and
// the challenges bound as before). So DEEP no longer trusts comp_z; it is the one
// compose proved was honestly formed from the frame.
#[test]
#[ignore]
fn the_transcript_compose_and_deep_are_wired_into_one_proof() {
    use crate::crypto::stark::air::{
        compose_inputs, deep_terms_query0, stark_prove_ext, stark_verify_ext, Air, AirExt,
        ComposeBoundary, ComposeCheck, DeepCheckExt, TranscriptCheck, TranscriptOp, WiredExt,
        WIDTH,
    };
    use crate::crypto::stark::field::Fp2;
    use alloc::boxed::Box;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    // Region 1: compose-at-z.
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut periodic = [Fp2::ZERO; 5];
    periodic.copy_from_slice(&ci.periodic_z[..5]);
    let mut coeffs = [Fp2::ZERO; 8];
    coeffs.copy_from_slice(&ci.coeffs[..8]);
    let boundaries: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, expected)| ComposeBoundary {
            col: *col,
            g_row: g.pow(*row as u64),
            expected: *expected,
        })
        .collect();
    let compose =
        ComposeCheck::new(window, periodic, coeffs, ci.z, ci.comp_z, g.pow(t - 1), t, boundaries);
    let ctrace = compose.trace();

    // Region 2: the DEEP check, holding comp_z (its composition term claim) as a
    // wireable trace cell.
    let (terms, dx, ddeep) = deep_terms_query0(&air, &proof, extra, &h);
    let deepck = DeepCheckExt::new(terms, dx, ddeep);
    let dtrace = deepck.trace();

    // Region 0: transcript derivation, up to the out-of-domain point.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], v: Fp| {
        ops.push(TranscriptOp::Absorb(v));
        st[0] = st[0] + v;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    let z_op = ops.len();
    squeeze(&mut ops, &mut st);
    squeeze(&mut ops, &mut st);
    let transcript = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    let regions: Vec<Box<dyn AirExt>> =
        alloc::vec![Box::new(transcript) as Box<dyn AirExt>, Box::new(compose), Box::new(deepck),];
    let l = 4usize;
    let t_height = 1usize << regions[0].log_trace_len();
    let c_off = t_height; // compose region offset
    let d_off = c_off + (1usize << regions[1].log_trace_len()); // DEEP region offset
    let span = (d_off + (1usize << regions[2].log_trace_len())).next_power_of_two();

    // wired columns: transcript squeeze lane (0), compose z+coeffs (22..39), compose
    // comp_z (54, 55), DEEP comp_z (4, 5).
    let mut wired_cols = alloc::vec![0usize];
    for c in 22..40 {
        wired_cols.push(c);
    }
    wired_cols.push(54);
    wired_cols.push(55);
    wired_cols.push(4);
    wired_cols.push(5);
    let k = wired_cols.len();
    let widx = |col: usize| -> usize { wired_cols.iter().position(|&c| c == col).unwrap() };
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // z and coefficients: transcript squeezes wire to compose columns.
    sigma.swap((z_op * l) * k, c_off * k + widx(22));
    sigma.swap(((z_op + 1) * l) * k, c_off * k + widx(23));
    for i in 0..8 {
        sigma.swap(((12 + 2 * i) * l) * k, c_off * k + widx(24 + 2 * i));
        sigma.swap(((12 + 2 * i + 1) * l) * k, c_off * k + widx(25 + 2 * i));
    }
    // comp_z: compose columns 54, 55 wire to DEEP columns 4, 5.
    sigma.swap(c_off * k + widx(54), d_off * k + widx(4));
    sigma.swap(c_off * k + widx(55), d_off * k + widx(5));

    let wired = WiredExt::new(regions, wired_cols, sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[ttrace, ctrace, dtrace]);
    let wproof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &wproof, 32, 8),
        "the transcript, compose, and DEEP regions were not consistently wired"
    );
}

// Five regions in one proof: the STARK transcript, composition, and DEEP (the
// computation half), plus the FRI transcript and the fold chain (the low-degree
// half), with the fold's challenges bound to the FRI transcript that squeezed them.
// So the fold no longer trusts its betas; they are the ones the FRI transcript
// proved.
#[test]
#[ignore]
fn the_full_verifier_computation_and_fold_are_wired_into_one_proof() {
    use crate::crypto::stark::air::{
        compose_inputs, deep_terms_query0, stark_prove_ext, stark_verify_ext, Air, AirExt,
        ComposeBoundary, ComposeCheck, DeepCheckExt, TraceFoldExt, TranscriptCheck, TranscriptOp,
        WiredExt, WIDTH,
    };
    use crate::crypto::stark::field::Fp2;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
    use alloc::boxed::Box;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    // Region 1: compose.
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut cperiodic = [Fp2::ZERO; 5];
    cperiodic.copy_from_slice(&ci.periodic_z[..5]);
    let mut coeffs = [Fp2::ZERO; 8];
    coeffs.copy_from_slice(&ci.coeffs[..8]);
    let bnds: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, e)| ComposeBoundary { col: *col, g_row: g.pow(*row as u64), expected: *e })
        .collect();
    let compose =
        ComposeCheck::new(window, cperiodic, coeffs, ci.z, ci.comp_z, g.pow(t - 1), t, bnds);
    let ctrace = compose.trace();

    // Region 2: DEEP.
    let (terms, dx, ddeep) = deep_terms_query0(&air, &proof, extra, &h);
    let deepck = DeepCheckExt::new(terms, dx, ddeep);
    let dtrace = deepck.trace();

    // Region 0: STARK transcript through z.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], v: Fp| {
        ops.push(TranscriptOp::Absorb(v));
        st[0] = st[0] + v;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    let z_op = ops.len();
    squeeze(&mut ops, &mut st);
    squeeze(&mut ops, &mut st);
    let transcript = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    // Region 3 + 4: the FRI transcript (interleaved absorb-root, squeeze-beta) and
    // the fold chain over the real proof's query 0.
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    let mut fs = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    let mut fri_st = [Fp::ZERO; WIDTH];
    let mut fri_ops: Vec<TranscriptOp> = Vec::new();
    for root in &fri.roots {
        fs.absorb_digest(root);
        betas.push(fs.challenge_fp2());
        for lane in root {
            absorb(&mut fri_ops, &mut fri_st, *lane);
        }
        squeeze(&mut fri_ops, &mut fri_st);
        squeeze(&mut fri_ops, &mut fri_st);
    }
    for value in &fri.final_layer {
        fs.absorb(value.c0);
        fs.absorb(value.c1);
    }
    assert!(fs.verify_pow(fri.pow_nonce, grind));
    let q0 = fs.challenge_index(n);
    let fri_transcript = TranscriptCheck::new(h.clone(), 2, fri_ops);
    let fttrace = fri_transcript.trace();

    let final_value = fri.final_layer[0];
    let base_omega = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let layers = &fri.queries[0].layers;
    let (mut a, mut b, mut x_inv, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (m, op) in layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let half = n >> (m + 1);
        let i = q0 % half;
        let x = (shift * base_omega.pow(i as u64)).pow(1u64 << m);
        x_inv.push(x.inv());
        dir.push(i >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);
    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new(log_layers, n_folds, x_inv, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(transcript) as Box<dyn AirExt>,
        Box::new(compose),
        Box::new(deepck),
        Box::new(fri_transcript),
        Box::new(fold),
    ];
    let l = 4usize;
    let off: Vec<usize> = {
        let mut v = Vec::new();
        let mut r = 0usize;
        for reg in &regions {
            v.push(r);
            r += 1usize << reg.log_trace_len();
        }
        v
    };
    let span = {
        let mut r = 0usize;
        for reg in &regions {
            r += 1usize << reg.log_trace_len();
        }
        r.next_power_of_two()
    };
    let (c_off, d_off, ft_off, f_off) = (off[1], off[2], off[3], off[4]);

    // wired columns: transcript squeeze lane (0), compose z+coeffs (22..39), compose
    // and DEEP comp_z (54,55 and 4,5), and the fold beta cells (columns 0,1 of the
    // fold region, but the fold shares low columns 0,1 with the transcript squeeze
    // lane 0, so beta.c1 uses column 1).
    let mut wired_cols = alloc::vec![0usize, 1];
    for c in 22..40 {
        wired_cols.push(c);
    }
    wired_cols.push(54);
    wired_cols.push(55);
    wired_cols.push(4);
    wired_cols.push(5);
    let k = wired_cols.len();
    let widx = |col: usize| -> usize { wired_cols.iter().position(|&c| c == col).unwrap() };
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // z and coefficients.
    sigma.swap((z_op * l) * k + widx(0), c_off * k + widx(22));
    sigma.swap(((z_op + 1) * l) * k + widx(0), c_off * k + widx(23));
    for i in 0..8 {
        sigma.swap(((12 + 2 * i) * l) * k + widx(0), c_off * k + widx(24 + 2 * i));
        sigma.swap(((12 + 2 * i + 1) * l) * k + widx(0), c_off * k + widx(25 + 2 * i));
    }
    // comp_z.
    sigma.swap(c_off * k + widx(54), d_off * k + widx(4));
    sigma.swap(c_off * k + widx(55), d_off * k + widx(5));
    // betas: FRI transcript squeeze (op 6m+4 for c0, 6m+5 for c1, lane 0) wire to the
    // fold's beta cells (row m, columns 0 and 1).
    for m in 0..n_folds {
        let b0_row = ft_off + (6 * m + 4) * l;
        let b1_row = ft_off + (6 * m + 5) * l;
        sigma.swap(b0_row * k + widx(0), (f_off + m) * k + widx(0));
        sigma.swap(b1_row * k + widx(0), (f_off + m) * k + widx(1));
    }

    let wired = WiredExt::new(regions, wired_cols, sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[ttrace, ctrace, dtrace, fttrace, ftrace]);
    let wproof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &wproof, 32, 8),
        "the five verifier regions were not consistently wired"
    );
}

// All six regions in one proof: the STARK transcript, composition, and DEEP; the
// FRI transcript and fold; and the Merkle authentication that binds the fold's
// opened value to the committed FRI codeword root. This is the whole verifier of a
// real Poseidon-committed proof, arithmetized and wired into one statement, with
// every challenge proven squeezed and every opened value authenticated.
#[test]
#[ignore]
fn the_full_recursive_verifier_is_wired_into_one_proof() {
    use crate::crypto::stark::air::{
        compose_inputs, deep_terms_query0, stark_prove_ext, stark_verify_ext, Air, AirExt,
        ComposeBoundary, ComposeCheck, DeepCheckExt, MultiMembership, Opening, TraceFoldExt,
        TranscriptCheck, TranscriptOp, WiredExt, WIDTH,
    };
    use crate::crypto::stark::field::Fp2;
    use crate::crypto::stark::poseidon_merkle::pack_ext;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
    use alloc::boxed::Box;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (air, proof) = poseidon_join_split_proof(&h, nq, grind, extra);
    let ci = compose_inputs(&air, &proof, extra, &h);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    // Region 1: compose.
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut cperiodic = [Fp2::ZERO; 5];
    cperiodic.copy_from_slice(&ci.periodic_z[..5]);
    let mut coeffs = [Fp2::ZERO; 8];
    coeffs.copy_from_slice(&ci.coeffs[..8]);
    let cbnds: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, e)| ComposeBoundary { col: *col, g_row: g.pow(*row as u64), expected: *e })
        .collect();
    let compose =
        ComposeCheck::new(window, cperiodic, coeffs, ci.z, ci.comp_z, g.pow(t - 1), t, cbnds);
    let ctrace = compose.trace();

    // Region 2: DEEP.
    let (terms, dx, ddeep) = deep_terms_query0(&air, &proof, extra, &h);
    let deepck = DeepCheckExt::new(terms, dx, ddeep);
    let dtrace = deepck.trace();

    // Region 0: STARK transcript through z.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], v: Fp| {
        ops.push(TranscriptOp::Absorb(v));
        st[0] = st[0] + v;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    let z_op = ops.len();
    squeeze(&mut ops, &mut st);
    squeeze(&mut ops, &mut st);
    let transcript = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    // Region 3 + 4: FRI transcript and fold.
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    let mut fs = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    let mut fri_st = [Fp::ZERO; WIDTH];
    let mut fri_ops: Vec<TranscriptOp> = Vec::new();
    for root in &fri.roots {
        fs.absorb_digest(root);
        betas.push(fs.challenge_fp2());
        for lane in root {
            absorb(&mut fri_ops, &mut fri_st, *lane);
        }
        squeeze(&mut fri_ops, &mut fri_st);
        squeeze(&mut fri_ops, &mut fri_st);
    }
    for value in &fri.final_layer {
        fs.absorb(value.c0);
        fs.absorb(value.c1);
    }
    assert!(fs.verify_pow(fri.pow_nonce, grind));
    let q0 = fs.challenge_index(n);
    let fri_transcript = TranscriptCheck::new(h.clone(), 2, fri_ops);
    let fttrace = fri_transcript.trace();

    let final_value = fri.final_layer[0];
    let base_omega = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let layers = &fri.queries[0].layers;
    let (mut a, mut b, mut x_inv, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (m, op) in layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let half = n >> (m + 1);
        let i = q0 % half;
        let x = (shift * base_omega.pow(i as u64)).pow(1u64 << m);
        x_inv.push(x.inv());
        dir.push(i >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);
    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new(log_layers, n_folds, x_inv, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);

    // Region 5: Merkle authentication of the fold's layer-0 opening against
    // roots[0], the committed DEEP codeword.
    let i0 = q0 % (n >> 1);
    let op0 = &fri.queries[0].layers[0];
    let siblings = op0.a_path.clone();
    let depth = siblings.len();
    let directions: Vec<bool> = (0..depth).map(|lv| (i0 >> lv) & 1 == 1).collect();
    let opening = Opening { leaf: pack_ext(op0.a), root: fri.roots[0], siblings, directions };
    let mem = MultiMembership::new(h.clone(), 2, alloc::vec![opening]);
    let (mrow, mcol) = mem.opened_cells()[0];
    let mtrace = mem.trace();

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(transcript) as Box<dyn AirExt>,
        Box::new(compose),
        Box::new(deepck),
        Box::new(fri_transcript),
        Box::new(fold),
        Box::new(mem),
    ];
    let l = 4usize;
    let off: Vec<usize> = {
        let mut v = Vec::new();
        let mut r = 0usize;
        for reg in &regions {
            v.push(r);
            r += 1usize << reg.log_trace_len();
        }
        v
    };
    let span = {
        let mut r = 0usize;
        for reg in &regions {
            r += 1usize << reg.log_trace_len();
        }
        r.next_power_of_two()
    };
    let (c_off, d_off, ft_off, f_off, m_off) = (off[1], off[2], off[3], off[4], off[5]);

    let mut wired_cols = alloc::vec![0usize, 1, 2, 3, 4, 5];
    for c in 22..40 {
        wired_cols.push(c);
    }
    wired_cols.push(54);
    wired_cols.push(55);
    // ensure the Merkle opened columns are wired.
    for c in [mcol, mcol + 1] {
        if !wired_cols.contains(&c) {
            wired_cols.push(c);
        }
    }
    let k = wired_cols.len();
    let widx = |col: usize| -> usize { wired_cols.iter().position(|&c| c == col).unwrap() };
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // z and coefficients.
    sigma.swap((z_op * l) * k + widx(0), c_off * k + widx(22));
    sigma.swap(((z_op + 1) * l) * k + widx(0), c_off * k + widx(23));
    for i in 0..8 {
        sigma.swap(((12 + 2 * i) * l) * k + widx(0), c_off * k + widx(24 + 2 * i));
        sigma.swap(((12 + 2 * i + 1) * l) * k + widx(0), c_off * k + widx(25 + 2 * i));
    }
    // comp_z.
    sigma.swap(c_off * k + widx(54), d_off * k + widx(4));
    sigma.swap(c_off * k + widx(55), d_off * k + widx(5));
    // betas.
    for m in 0..n_folds {
        sigma.swap((ft_off + (6 * m + 4) * l) * k + widx(0), (f_off + m) * k + widx(0));
        sigma.swap((ft_off + (6 * m + 5) * l) * k + widx(0), (f_off + m) * k + widx(1));
    }
    // the fold's layer-0 opened value equals the Merkle-authenticated leaf.
    sigma.swap((f_off) * k + widx(2), (m_off + mrow) * k + widx(mcol));
    sigma.swap((f_off) * k + widx(3), (m_off + mrow) * k + widx(mcol + 1));

    let wired = WiredExt::new(regions, wired_cols, sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[ttrace, ctrace, dtrace, fttrace, ftrace, mtrace]);
    let wproof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &wproof, 32, 8),
        "the full recursive verifier's six regions were not consistently wired"
    );
}

// The publics-binding reference: a batch's K*11 per-intent public words are absorbed
// into the transcript's inject column (the FS input column the pool reads), proven in
// circuit, with word j of intent i landing at row (i*11+j)*l. This is the layout the
// pool's settleBatch extraction indexes; the production vector rides this same column,
// with the words being the real batch publics the inner join-split proof absorbs.
#[test]
#[ignore]
fn gen_publics_bound_reference() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Air, TranscriptCheck, TranscriptOp, WIDTH,
    };
    use alloc::string::String;

    let h = hasher();
    let k_intents = 2usize;
    let words = 11usize;
    let l = 4usize; // permutation rounds

    // Representative per-intent publics in the frozen 11-word order.
    let mut publics: Vec<Fp> = Vec::with_capacity(k_intents * words);
    for i in 0..k_intents {
        for j in 0..words {
            publics.push(Fp::from_u64(0x9000 + (i * words + j) as u64));
        }
    }

    // The transcript absorbs every public word, then squeezes a binding challenge so
    // the challenge depends on the whole batch statement.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    for &p in &publics {
        ops.push(TranscriptOp::Absorb(p));
        st[0] = st[0] + p;
        st = h.permute(st);
    }
    ops.push(TranscriptOp::Squeeze(st[0]));
    st = h.permute(st);
    ops.push(TranscriptOp::Squeeze(st[0]));

    let tc = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = tc.trace();
    let proof = stark_prove_ext(&tc, &ttrace, 32, 8);
    assert!(stark_verify_ext(&tc, &proof, 32, 8), "the publics-bound transcript was rejected");

    // The FS input column is the inject periodic column (index WIDTH); word j of
    // intent i is at row (i*11+j)*l in that column.
    let mut pubs_json = String::from("[");
    for (idx, p) in publics.iter().enumerate() {
        if idx > 0 {
            pubs_json.push(',');
        }
        let (i, j) = (idx / words, idx % words);
        pubs_json.push_str(&alloc::format!(
            "[{},{},{},\"{}\"]",
            i,
            j,
            (i * words + j) * l,
            p.value()
        ));
    }
    pubs_json.push(']');

    let bytes = crate::stark_selftest_gen::serialize(&proof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"artifact\": \"publics-bound-reference\",\n  \"note\": \"Reference for the pool publics binding. K*11 per-intent public words absorbed into the transcript inject column (the FS input column), proven in circuit. word j of intent i is at row (i*11+j)*l of the FS input column. The production vector is this same column carrying the real batch publics that the inner join-split proof absorbs, wired into the assembled recursion.\",\n  \"l\": {},\n  \"fs_input_column_index\": {},\n  \"k_intents\": {},\n  \"words_per_intent\": {},\n  \"trace_width\": {},\n  \"publics\": {},\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        l, WIDTH, k_intents, words, tc.trace_width(), pubs_json, bytes.len(),
        crate::stark_selftest_gen::hex(&bytes)
    );
    std::fs::write("/Users/ek/Desktop/NOX-SmartContract/spec/publics-bound-reference.json", &json)
        .expect("write");
    std::println!(
        "wrote publics-bound reference: {} publics, {} proof bytes",
        publics.len(),
        bytes.len()
    );
}

// The production recursive vector: the inner join-split proof absorbs the batch's
// K*11 public words into its transcript (bound by Fiat-Shamir), and the full
// six-region recursion is assembled over it, with the STARK transcript region
// replaying those publics so they land in the FS input column at (i*11+j)*l. This
// emits the vector the pool's settleBatch flips against. Representative publics in
// the frozen 11-word order; the real batch swaps them, the layout unchanged.
#[test]
#[ignore]
fn gen_production_recursive_vector() {
    use crate::crypto::stark::air::{
        compose_inputs_pub, deep_terms_query0_pub, stark_prove_ext_blown,
        stark_prove_poseidon_ext_pub, stark_verify_ext_blown, Accumulator, Air, AirExt,
        ComposeBoundary, ComposeCheck, DeepCheckExt, GpGroup, MultiMembership, Opening, RangeCheck,
        TraceFoldExt, TranscriptCheck, TranscriptOp, WiredExt, WiredMultiExt, WIDTH,
    };
    use crate::crypto::stark::field::Fp2;
    use crate::crypto::stark::poseidon_merkle::pack_ext;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
    use alloc::boxed::Box;
    use alloc::string::String;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let l = 4usize;
    let words = 11usize;
    let k_intents = 2usize;

    // The batch publics in the frozen order: noteRoot, assocRoot, nullifier0,
    // nullifier1, outCommit0, outCommit1, public_amount, fee, asset_id,
    // recipient_lo, recipient_hi. Representative values.
    let mut publics: Vec<Fp> = Vec::with_capacity(k_intents * words);
    for i in 0..k_intents {
        for j in 0..words {
            publics.push(Fp::from_u64(0xA000 + (i * words + j) as u64));
        }
    }
    let pub_len = publics.len();

    // The inner join-split, proven while absorbing the publics.
    let regions0: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sig0: Vec<usize> = (0..32).collect();
    sig0.swap(1, 8);
    let inner = WiredExt::new(regions0, alloc::vec![0], sig0, Fp::from_u64(5), Fp::from_u64(7));
    let neg = |x: u64| -> Fp { Fp::ZERO - Fp::from_u64(x) };
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
    let iw = inner.trace(&[cons, rng]);
    let proof = stark_prove_poseidon_ext_pub(&inner, &iw, nq, grind, extra, &h, &publics);
    let air = &inner;
    let ci = compose_inputs_pub(air, &proof, extra, &h, &publics);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    // Region 1: compose.
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut cp = [Fp2::ZERO; 5];
    cp.copy_from_slice(&ci.periodic_z[..5]);
    let mut cf = [Fp2::ZERO; 8];
    cf.copy_from_slice(&ci.coeffs[..8]);
    let cbnds: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, e)| ComposeBoundary { col: *col, g_row: g.pow(*row as u64), expected: *e })
        .collect();
    let compose = ComposeCheck::new(window, cp, cf, ci.z, ci.comp_z, g.pow(t - 1), t, cbnds);
    let ctrace = compose.trace();

    // Region 2: DEEP.
    let (terms, dx, ddeep) = deep_terms_query0_pub(air, &proof, extra, &h, &publics);
    let deepck = DeepCheckExt::new(terms, dx, ddeep);
    let dtrace = deepck.trace();

    // Region 0: STARK transcript, publics first, then the proof's own sequence.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], val: Fp| {
        ops.push(TranscriptOp::Absorb(val));
        st[0] = st[0] + val;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for &p in &publics {
        absorb(&mut ops, &mut st, p);
    }
    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    let z_op = ops.len();
    squeeze(&mut ops, &mut st);
    squeeze(&mut ops, &mut st);
    let transcript = TranscriptCheck::new(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    // Region 3 + 4: FRI transcript and fold.
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;
    let mut fs = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    let mut fst = [Fp::ZERO; WIDTH];
    let mut fops: Vec<TranscriptOp> = Vec::new();
    for root in &fri.roots {
        fs.absorb_digest(root);
        betas.push(fs.challenge_fp2());
        for lane in root {
            absorb(&mut fops, &mut fst, *lane);
        }
        squeeze(&mut fops, &mut fst);
        squeeze(&mut fops, &mut fst);
    }
    for value in &fri.final_layer {
        fs.absorb(value.c0);
        fs.absorb(value.c1);
    }
    assert!(fs.verify_pow(fri.pow_nonce, grind));
    let q0 = fs.challenge_index(n);
    let fri_transcript = TranscriptCheck::new(h.clone(), 2, fops);
    let fttrace = fri_transcript.trace();

    let final_value = fri.final_layer[0];
    let bo = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let layers = &fri.queries[0].layers;
    let (mut a, mut b, mut xi, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (m, op) in layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let half = n >> (m + 1);
        let ix = q0 % half;
        xi.push((shift * bo.pow(ix as u64)).pow(1u64 << m).inv());
        dir.push(ix >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);
    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new(log_layers, n_folds, xi, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);

    // Region 5: Merkle authentication.
    let i0 = q0 % (n >> 1);
    let op0 = &fri.queries[0].layers[0];
    let sibs = op0.a_path.clone();
    let depth = sibs.len();
    let dirs: Vec<bool> = (0..depth).map(|lv| (i0 >> lv) & 1 == 1).collect();
    let mem = MultiMembership::new(
        h.clone(),
        2,
        alloc::vec![Opening {
            leaf: pack_ext(op0.a),
            root: fri.roots[0],
            siblings: sibs,
            directions: dirs.clone()
        }],
    );
    let (mrow, mcol) = mem.opened_cells()[0];
    let mtrace = mem.trace();

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(transcript) as Box<dyn AirExt>,
        Box::new(compose),
        Box::new(deepck),
        Box::new(fri_transcript),
        Box::new(fold),
        Box::new(mem),
    ];
    let off: Vec<usize> = {
        let mut vv = Vec::new();
        let mut r = 0usize;
        for reg in &regions {
            vv.push(r);
            r += 1usize << reg.log_trace_len();
        }
        vv
    };
    let span = {
        let mut r = 0usize;
        for reg in &regions {
            r += 1usize << reg.log_trace_len();
        }
        r.next_power_of_two()
    };
    let (c_off, d_off, ft_off, f_off, m_off) = (off[1], off[2], off[3], off[4], off[5]);

    // The copy constraint is split into independent grand-product groups so no one
    // permutation spans every wired column. Each group binds a handful of cells and
    // carries its own running product, keeping the AIR degree at the region maximum
    // instead of the total wired-column count. A group's sigma is the identity over
    // its own cells with the shared values swapped; identity cells cancel, so only
    // the explicit swaps must hold equal.
    let group = |wcols: Vec<usize>, swaps: &[(usize, usize, usize, usize)]| -> GpGroup {
        let kk = wcols.len();
        let mut sig: Vec<usize> = (0..span * kk).collect();
        for &(ra, ca, rb, cb) in swaps {
            let ia = wcols.iter().position(|&c| c == ca).unwrap();
            let ib = wcols.iter().position(|&c| c == cb).unwrap();
            sig.swap(ra * kk + ia, rb * kk + ib);
        }
        GpGroup { wired_cols: wcols, sigma: sig, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
    };

    let mut groups: Vec<GpGroup> = Vec::new();
    // z: out-of-domain point squeezed in the STARK transcript == compose input.
    groups.push(group(
        alloc::vec![0, 22, 23],
        &[(z_op * l, 0, c_off, 22), ((z_op + 1) * l, 0, c_off, 23)],
    ));
    // coefficients: sixteen Fp columns split into four groups of four.
    for grp in 0..4 {
        let bc = 24 + 4 * grp;
        let bo = pub_len + 12 + 4 * grp;
        groups.push(group(
            alloc::vec![0, bc, bc + 1, bc + 2, bc + 3],
            &[
                (bo * l, 0, c_off, bc),
                ((bo + 1) * l, 0, c_off, bc + 1),
                ((bo + 2) * l, 0, c_off, bc + 2),
                ((bo + 3) * l, 0, c_off, bc + 3),
            ],
        ));
    }
    // comp_z: composition value at z == DEEP composition claim.
    groups.push(group(alloc::vec![54, 55, 4, 5], &[(c_off, 54, d_off, 4), (c_off, 55, d_off, 5)]));
    // betas: fold challenges squeezed in the FRI transcript == fold inputs.
    let mut beta_swaps: Vec<(usize, usize, usize, usize)> = Vec::new();
    for m in 0..n_folds {
        beta_swaps.push((ft_off + (6 * m + 4) * l, 0, f_off + m, 0));
        beta_swaps.push((ft_off + (6 * m + 5) * l, 0, f_off + m, 1));
    }
    groups.push(group(alloc::vec![0, 1], &beta_swaps));
    // fold opened value == authenticated Merkle leaf.
    groups.push(group(
        alloc::vec![2, 3, mcol, mcol + 1],
        &[(f_off, 2, m_off + mrow, mcol), (f_off, 3, m_off + mrow, mcol + 1)],
    ));

    let wired = WiredMultiExt::new(regions, groups);
    let witness = wired.trace(&[ttrace, ctrace, dtrace, fttrace, ftrace, mtrace]);

    // Emit the width-79 AIR structure first (fast, no proof): the spec to build the
    // production verifier and the on-chain deterministic-statement recomputation.
    {
        let (ltl, tw, deg, ntr) = (
            wired.log_trace_len(),
            wired.trace_width(),
            wired.constraint_degree(),
            wired.num_transition(),
        );
        let bnd = wired.boundary();
        let nper = wired.periodic_columns().len();
        let tt = 1u64 << ltl;
        let bound = ((deg as u64 * tt) as usize).next_power_of_two();
        let dn = (2 * bound) << 3; // extra_blowup_bits = 3 -> rate 1/16
        let log_dn = dn.trailing_zeros();
        let fri_log_blowup = log_dn - bound.trailing_zeros();
        let mut regs = String::from("[");
        for (ri, o) in off.iter().enumerate() {
            if ri > 0 {
                regs.push(',');
            }
            let hgt = if ri + 1 < off.len() { off[ri + 1] - o } else { span - o };
            regs.push_str(&alloc::format!(
                "{{\"region\":{},\"offset\":{},\"height\":{}}}",
                ri,
                o,
                hgt
            ));
        }
        regs.push(']');
        let mut bstr = String::from("[");
        for (bi, (c, r, val)) in bnd.iter().enumerate() {
            if bi > 0 {
                bstr.push(',');
            }
            bstr.push_str(&alloc::format!("[{},{},\"{}\"]", c, r, val.value()));
        }
        bstr.push(']');
        let sjson = alloc::format!(
            "{{\n  \"artifact\": \"production-air-structure\",\n  \"note\": \"The recursion AIR: six regions (0 STARK transcript, 1 compose, 2 DEEP, 3 FRI transcript, 4 fold, 5 Merkle) stacked, the copy constraint split across eight grand-product columns (z, four coefficient groups, comp_z, betas, fold-vs-Merkle) so the constraint degree stays at the region maximum instead of the total wired-column count. Build the production verifier and the deterministic-statement recomputation against this. Deployment soundness is rate 1/16, 32 queries, 16 grind bits = 128-bit conjectured.\",\n  \"log_trace_len\": {}, \"trace_width\": {}, \"constraint_degree\": {}, \"num_transition\": {},\n  \"n_queries\": 32, \"grind_bits\": 16, \"extra_blowup_bits\": 3, \"fri_log_blowup\": {}, \"log_eval_domain\": {},\n  \"num_periodic_columns\": {}, \"num_boundaries\": {},\n  \"regions\": {},\n  \"boundaries\": {}\n}}\n",
            ltl, tw, deg, ntr, fri_log_blowup, log_dn, nper, bnd.len(), regs, bstr
        );
        std::fs::write(
            "/Users/ek/Desktop/NOX-SmartContract/spec/production-air-structure.json",
            &sjson,
        )
        .expect("write structure");
        std::println!(
            "wrote AIR structure: width {}, {} periodic, {} boundaries, log_eval_domain {}",
            tw,
            nper,
            bnd.len(),
            log_dn
        );
    }

    // The deployment proof: rate 1/16, 16 grind bits = 128-bit conjectured.
    let wproof = stark_prove_ext_blown(&wired, &witness, 32, 16, 3);
    assert!(
        stark_verify_ext_blown(&wired, &wproof, 32, 16, 3),
        "the production recursive vector does not verify"
    );

    // The publics ride the transcript region's inject column at (i*11+j)*l.
    let mut pubs_json = String::from("[");
    for (idx, p) in publics.iter().enumerate() {
        if idx > 0 {
            pubs_json.push(',');
        }
        let (i, j) = (idx / words, idx % words);
        pubs_json.push_str(&alloc::format!(
            "[{},{},{},\"{}\"]",
            i,
            j,
            (i * words + j) * l,
            p.value()
        ));
    }
    pubs_json.push(']');
    let bytes = crate::stark_selftest_gen::serialize(&wproof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"artifact\": \"production-recursive-vector\",\n  \"note\": \"The full six-region recursion over a real Poseidon-committed join-split proof that absorbed the batch's K*11 publics. Every challenge is proven squeezed, every opened value authenticated, and the publics ride the transcript inject column (the FS input column) at (i*11+j)*l. settleBatch: verify -> extract at column {}, rows (i*11+j)*{} -> gate. Representative publics in the frozen 11-word order; the real batch swaps them, the layout unchanged.\",\n  \"l\": {}, \"fs_input_column_index\": {}, \"k_intents\": {}, \"words_per_intent\": {},\n  \"n_queries\": 32, \"grind_bits\": 16, \"extra_blowup_bits\": 3, \"fri_log_blowup\": 4, \"trace_width\": {},\n  \"publics\": {},\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        WIDTH, l, l, WIDTH, k_intents, words, wired.trace_width(), pubs_json, bytes.len(),
        crate::stark_selftest_gen::hex(&bytes)
    );
    std::fs::write(
        "/Users/ek/Desktop/NOX-SmartContract/spec/production-recursive-vector.json",
        &json,
    )
    .expect("write");
    std::println!(
        "wrote production vector: {} publics, {} proof bytes, {} regions",
        pub_len,
        bytes.len(),
        6
    );

    // Intermediate reference oracle: replay the outer verify on the emitted proof
    // and dump the known-good field elements each verifier stage must reproduce, so
    // the contracts team gates each region against exact values, not just the final
    // accept. Everything here mirrors stark_verify_ext_blown + fri_verify_ext.
    {
        use crate::crypto::stark::air::compose_ext;
        use crate::crypto::stark::fri::root_of_unity;
        use crate::crypto::stark::poly::eval_lagrange_ext;
        use crate::crypto::stark::transcript::Transcript;

        let log_t = wired.log_trace_len();
        let tt = 1usize << log_t;
        let widthw = wired.trace_width();
        let deg = wired.constraint_degree().max(1);
        let bound = (deg * tt).next_power_of_two();
        let nn = (2 * bound) << 3;
        let log_n = nn.trailing_zeros();
        let wsz = wired.window_size();
        let gg = root_of_unity(log_t);
        let omega = root_of_unity(log_n);
        let shift = Fp::from_u64(7);
        let ncoeff = wired.num_transition() + wired.boundary().len();

        // Outer STARK transcript, exactly as the verifier draws it.
        let mut ts = Transcript::new(b"NONOS-STARK-EXT");
        ts.absorb_digest(&wproof.trace_root);
        let coeffs: Vec<Fp2> = (0..ncoeff).map(|_| ts.challenge_fp2()).collect();
        ts.absorb_digest(&wproof.comp_root);
        let shift_n = Fp2::from_base(shift.pow(nn as u64));
        let mut z = ts.challenge_fp2();
        while z.pow(nn as u64) == shift_n || z.pow(tt as u64) == Fp2::ONE {
            z = ts.challenge_fp2();
        }
        for value in &wproof.ood_frame {
            ts.absorb_fp(value.c0);
            ts.absorb_fp(value.c1);
        }
        let deep_coeffs: Vec<Fp2> = (0..widthw * wsz + 1).map(|_| ts.challenge_fp2()).collect();

        // Composition at z and its per-constraint breakdown.
        let h_pts: Vec<Fp> = {
            let mut v = Vec::with_capacity(tt);
            let mut p = Fp::ONE;
            for _ in 0..tt {
                v.push(p);
                p = p * gg;
            }
            v
        };
        let periodic_z: Vec<Fp2> =
            wired.periodic_columns().iter().map(|col| eval_lagrange_ext(&h_pts, col, z)).collect();
        let comp_z = compose_ext(&wired, gg, z, &wproof.ood_frame, &periodic_z, &coeffs);
        let transition_z = wired.transition_ext(&wproof.ood_frame, &periodic_z);

        // FRI transcript: fold challenges then the FRI query positions.
        let mut fts = Transcript::new(b"NONOS-STARK-FRI-EXT");
        let mut betas: Vec<Fp2> = Vec::new();
        for root in &wproof.fri.roots {
            fts.absorb_digest(root);
            betas.push(fts.challenge_fp2());
        }
        for value in &wproof.fri.final_layer {
            fts.absorb_fp(value.c0);
            fts.absorb_fp(value.c1);
        }
        assert!(fts.verify_pow(wproof.fri.pow_nonce, 16));
        let fri_qidx: Vec<usize> = (0..32).map(|_| fts.challenge_index(nn)).collect();

        // Outer consistency query positions.
        ts.absorb_digest(&wproof.fri.roots[0]);
        let cons_qidx: Vec<usize> = (0..32).map(|_| ts.challenge_index(nn)).collect();

        let p0 = cons_qidx[0];
        let x0 = shift * omega.pow(p0 as u64);
        let q0 = &wproof.queries[0];
        let fq0 = &wproof.fri.queries[0];

        let fp = |v: Fp| alloc::format!("\"{}\"", v.value());
        let fp2 = |v: Fp2| alloc::format!("[\"{}\",\"{}\"]", v.c0.value(), v.c1.value());
        let fp2s = |vs: &[Fp2]| {
            let mut s = String::from("[");
            for (i, v) in vs.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&fp2(*v));
            }
            s.push(']');
            s
        };
        let fps = |vs: &[Fp]| {
            let mut s = String::from("[");
            for (i, v) in vs.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&fp(*v));
            }
            s.push(']');
            s
        };
        let us = |vs: &[usize]| {
            let mut s = String::from("[");
            for (i, v) in vs.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&alloc::format!("{}", v));
            }
            s.push(']');
            s
        };
        let mut layers = String::from("[");
        for (i, lyr) in fq0.layers.iter().enumerate() {
            if i > 0 {
                layers.push(',');
            }
            layers.push_str(&alloc::format!("{{\"a\":{},\"b\":{}}}", fp2(lyr.a), fp2(lyr.b)));
        }
        layers.push(']');

        // Merkle ground truth for the fold-versus-leaf group (g7): the opened cell
        // column must be derived from the query's leaf-level direction, not pinned.
        let mut dirs_s = String::from("[");
        for (i, d) in dirs.iter().enumerate() {
            if i > 0 {
                dirs_s.push(',');
            }
            dirs_s.push_str(if *d { "true" } else { "false" });
        }
        dirs_s.push(']');

        let ntr = wired.num_transition();
        let ijson = alloc::format!(
            "{{\n  \"artifact\": \"production-intermediates\",\n  \"note\": \"Known-good field elements from replaying the outer verify on the emitted vector. Gate each verifier stage against these, not only the final accept. transition_z is the exact output of the width-{tw} transition_ext at z: the first {region_tr} entries are the region constraints (selector-weighted at z), the last 8 are the grand-product terms; your transition_ext must match this array elementwise. comp_z is the batched composition your compose_ext must reproduce. periodic_z is the {nper}-column periodic recomputation at z. fri_query_indices come from the FRI transcript, consistency_query_indices from the outer one. Fp as decimal string, Fp2 as [c0,c1].\",\n  \"log_trace_len\": {ltl}, \"trace_width\": {tw}, \"window_size\": {wsz}, \"log_eval_domain\": {ln}, \"num_transition\": {ntr}, \"num_coeffs\": {nc}, \"num_periodic\": {nper},\n  \"z\": {z},\n  \"coeffs\": {coeffs},\n  \"deep_coeffs\": {deepc},\n  \"betas\": {betas},\n  \"fri_query_indices\": {friq},\n  \"consistency_query_indices\": {consq},\n  \"comp_z\": {compz},\n  \"transition_z\": {trz},\n  \"periodic_z\": {perz},\n  \"query0\": {{ \"index\": {p0}, \"x\": {x0}, \"trace_row\": {row}, \"comp\": {qcomp}, \"deep\": {qdeep} }},\n  \"fri_query0\": {{ \"layers\": {layers}, \"final_value\": {fv} }},\n  \"merkle_query0\": {{ \"note\": \"g7 (fold==Merkle leaf) opened cell. Derive opened_col from the leaf-level direction, do not pin it. inner_leaf_index i0, directions per level (LSB first), opened_row/opened_col are the ground truth.\", \"inner_leaf_index\": {i0}, \"directions\": {dirs}, \"opened_row\": {mrow}, \"opened_col\": {mcol} }}\n}}\n",
            region_tr = ntr - 8,
            nper = periodic_z.len(),
            ltl = log_t,
            tw = widthw,
            wsz = wsz,
            ln = log_n,
            ntr = ntr,
            nc = ncoeff,
            z = fp2(z),
            coeffs = fp2s(&coeffs),
            deepc = fp2s(&deep_coeffs),
            betas = fp2s(&betas),
            friq = us(&fri_qidx),
            consq = us(&cons_qidx),
            compz = fp2(comp_z),
            trz = fp2s(&transition_z),
            perz = fp2s(&periodic_z),
            p0 = p0,
            x0 = fp(x0),
            row = fps(&q0.trace),
            qcomp = fp2(q0.comp),
            qdeep = fp2(q0.deep),
            layers = layers,
            fv = fp2(wproof.fri.final_layer[0]),
            i0 = i0,
            dirs = dirs_s,
            mrow = mrow,
            mcol = mcol,
        );
        std::fs::write(
            "/Users/ek/Desktop/NOX-SmartContract/spec/reference/intermediates.json",
            &ijson,
        )
        .expect("write intermediates");
        std::println!(
            "wrote intermediates: z, {} coeffs, {} deep_coeffs, {} betas, comp_z, {} transition_z, {} periodic_z",
            coeffs.len(), deep_coeffs.len(), betas.len(), transition_z.len(), periodic_z.len()
        );
    }
}

// The gap-closed, instance-independent recursion assembly. All six regions in
// witness form (transcript inject, DEEP terms, Merkle path on the trace), and
// region 5 authenticates the full inner opening set (the FRI leaf for the fold,
// then deep, comp, and every trace column at the consistency index), so no opened
// value feeding the DEEP batch is trusted and the AIR carries no instance-specific
// periodic or boundary data. Built incrementally: this step assembles the regions
// and proves the trace is internally consistent, before the grand-product bindings.
#[test]
#[ignore]
fn the_gapclosed_assembly_accepts() {
    use crate::crypto::stark::air::{
        compose_inputs_pub, deep_terms_query0_pub, query_openings_query0, stark_prove_ext,
        stark_prove_poseidon_ext_pub, stark_verify_ext, Accumulator, Air, AirExt, ComposeBoundary,
        ComposeCheck, DeepCheckExt, GpGroup, MultiMembership, Opening, RangeCheck, TraceFoldExt,
        TranscriptCheck, TranscriptOp, WiredExt, WiredMultiExt, WIDTH,
    };
    use crate::crypto::stark::field::Fp2;
    use crate::crypto::stark::fri::root_of_unity;
    use crate::crypto::stark::poseidon_merkle::pack_ext;
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
    use alloc::boxed::Box;

    let h = hasher();
    let (nq, grind, extra) = (32usize, 16u32, 3u32);
    let (words, k_intents) = (11usize, 2usize);
    let mut publics: Vec<Fp> = Vec::with_capacity(k_intents * words);
    for i in 0..k_intents {
        for j in 0..words {
            publics.push(Fp::from_u64(0xA000 + (i * words + j) as u64));
        }
    }

    // Inner join-split, proven while absorbing the publics.
    let regions0: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sig0: Vec<usize> = (0..32).collect();
    sig0.swap(1, 8);
    let inner = WiredExt::new(regions0, alloc::vec![0], sig0, Fp::from_u64(5), Fp::from_u64(7));
    let neg = |x: u64| -> Fp { Fp::ZERO - Fp::from_u64(x) };
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
    let iw = inner.trace(&[cons, rng]);
    let proof = stark_prove_poseidon_ext_pub(&inner, &iw, nq, grind, extra, &h, &publics);
    let air = &inner;
    let ci = compose_inputs_pub(air, &proof, extra, &h, &publics);
    let t = 1u64 << air.log_trace_len();
    let g = root_of_unity(air.log_trace_len());

    // Region 1: compose.
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&proof.ood_frame[..6]);
    let mut cp = [Fp2::ZERO; 5];
    cp.copy_from_slice(&ci.periodic_z[..5]);
    let mut cf = [Fp2::ZERO; 8];
    cf.copy_from_slice(&ci.coeffs[..8]);
    let cbnds: Vec<ComposeBoundary> = air
        .boundary()
        .iter()
        .map(|(col, row, e)| ComposeBoundary { col: *col, g_row: g.pow(*row as u64), expected: *e })
        .collect();
    let compose =
        ComposeCheck::new_witness(window, cp, cf, ci.z, ci.comp_z, g.pow(t - 1), t, cbnds);
    let ctrace = compose.trace();

    // Region 2: DEEP, witness form.
    let (terms, dx, ddeep) = deep_terms_query0_pub(air, &proof, extra, &h, &publics);
    let n_terms = terms.len();
    let deepck = DeepCheckExt::new_witness(terms, dx, ddeep);
    let dtrace = deepck.trace();

    // Region 0: STARK transcript, witness form.
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    let absorb = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], val: Fp| {
        ops.push(TranscriptOp::Absorb(val));
        st[0] = st[0] + val;
        *st = h.permute(*st);
    };
    let squeeze = |ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]| {
        let c = st[0];
        ops.push(TranscriptOp::Squeeze(c));
        *st = h.permute(*st);
    };
    for &p in &publics {
        absorb(&mut ops, &mut st, p);
    }
    for root in &proof.trace_roots {
        for lane in root {
            absorb(&mut ops, &mut st, *lane);
        }
    }
    for _ in 0..ci.coeffs.len() * 2 {
        squeeze(&mut ops, &mut st);
    }
    for lane in &proof.comp_root {
        absorb(&mut ops, &mut st, *lane);
    }
    let z_op = ops.len();
    squeeze(&mut ops, &mut st);
    squeeze(&mut ops, &mut st);
    // Continue the replay through the DEEP coefficients so they exist as squeeze
    // cells to bind the DEEP region's coefficients to.
    for value in &proof.ood_frame {
        absorb(&mut ops, &mut st, value.c0);
        absorb(&mut ops, &mut st, value.c1);
    }
    let deep_coeff_op = ops.len();
    for _ in 0..n_terms {
        squeeze(&mut ops, &mut st);
        squeeze(&mut ops, &mut st);
    }
    let transcript = TranscriptCheck::new_witness(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    // Region 3 + 4: FRI transcript (witness) and fold.
    let fri = &proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;
    let mut fs = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    let mut fst = [Fp::ZERO; WIDTH];
    let mut fops: Vec<TranscriptOp> = Vec::new();
    for root in &fri.roots {
        fs.absorb_digest(root);
        betas.push(fs.challenge_fp2());
        for lane in root {
            absorb(&mut fops, &mut fst, *lane);
        }
        squeeze(&mut fops, &mut fst);
        squeeze(&mut fops, &mut fst);
    }
    for value in &fri.final_layer {
        fs.absorb(value.c0);
        fs.absorb(value.c1);
    }
    assert!(fs.verify_pow(fri.pow_nonce, grind));
    let q0 = fs.challenge_index(n);
    let fri_transcript = TranscriptCheck::new_witness(h.clone(), 2, fops);
    let fttrace = fri_transcript.trace();

    let final_value = fri.final_layer[0];
    let bo = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let layers = &fri.queries[0].layers;
    let (mut a, mut b, mut xi, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (m, op) in layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let half = n >> (m + 1);
        let ix = q0 % half;
        xi.push((shift * bo.pow(ix as u64)).pow(1u64 << m).inv());
        dir.push(ix >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);
    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new(log_layers, n_folds, xi, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);

    // Region 5: batched authentication. The FRI leaf (for the fold), then the whole
    // consistency opening set: deep, comp, and every trace column, each against its
    // committed root, all equal depth.
    let i0 = q0 % (n >> 1);
    let op0 = &fri.queries[0].layers[0];
    let sibs = op0.a_path.clone();
    let depth = sibs.len();
    let dirs: Vec<bool> = (0..depth).map(|lv| (i0 >> lv) & 1 == 1).collect();
    let mut openings = alloc::vec![Opening {
        leaf: pack_ext(op0.a),
        root: fri.roots[0],
        siblings: sibs,
        directions: dirs,
    }];
    openings.extend(query_openings_query0(air, &proof, extra, &h, &publics));
    let mem = MultiMembership::new_witness(h.clone(), 2, openings);
    let mtrace = mem.trace();
    // opened_cells[0] is the FRI leaf (fold), [1] deep, [2] comp, [3+c] trace[c].
    let ocells = mem.opened_cells();
    let (mrow, mcol) = ocells[0];
    let pub_len = publics.len();
    let l = 4usize;

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(transcript) as Box<dyn AirExt>,
        Box::new(compose),
        Box::new(deepck),
        Box::new(fri_transcript),
        Box::new(fold),
        Box::new(mem),
    ];
    let off: Vec<usize> = {
        let mut vv = Vec::new();
        let mut r = 0usize;
        for reg in &regions {
            vv.push(r);
            r += 1usize << reg.log_trace_len();
        }
        vv
    };
    let span = {
        let mut r = 0usize;
        for reg in &regions {
            r += 1usize << reg.log_trace_len();
        }
        r.next_power_of_two()
    };
    let (c_off, d_off, ft_off, f_off, m_off) = (off[1], off[2], off[3], off[4], off[5]);

    let group = |wcols: Vec<usize>, swaps: &[(usize, usize, usize, usize)]| -> GpGroup {
        let kk = wcols.len();
        let mut sig: Vec<usize> = (0..span * kk).collect();
        for &(ra, ca, rb, cb) in swaps {
            let ia = wcols.iter().position(|&c| c == ca).unwrap();
            let ib = wcols.iter().position(|&c| c == cb).unwrap();
            sig.swap(ra * kk + ia, rb * kk + ib);
        }
        GpGroup { wired_cols: wcols, sigma: sig, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
    };

    // Step 2a: the existing eight groups, unchanged wired columns, offsets recomputed
    // against the larger span. z, the sixteen coefficients, comp_z, the betas, and
    // the fold value against the FRI-leaf opening (now opening zero of the batch).
    let mut groups: Vec<GpGroup> = Vec::new();
    groups.push(group(
        alloc::vec![0, 22, 23],
        &[(z_op * l, 0, c_off, 22), ((z_op + 1) * l, 0, c_off, 23)],
    ));
    for grp in 0..4 {
        let bc = 24 + 4 * grp;
        let bo = pub_len + 12 + 4 * grp;
        groups.push(group(
            alloc::vec![0, bc, bc + 1, bc + 2, bc + 3],
            &[
                (bo * l, 0, c_off, bc),
                ((bo + 1) * l, 0, c_off, bc + 1),
                ((bo + 2) * l, 0, c_off, bc + 2),
                ((bo + 3) * l, 0, c_off, bc + 3),
            ],
        ));
    }
    groups.push(group(alloc::vec![54, 55, 4, 5], &[(c_off, 54, d_off, 4), (c_off, 55, d_off, 5)]));
    let mut beta_swaps: Vec<(usize, usize, usize, usize)> = Vec::new();
    for m in 0..n_folds {
        beta_swaps.push((ft_off + (6 * m + 4) * l, 0, f_off + m, 0));
        beta_swaps.push((ft_off + (6 * m + 5) * l, 0, f_off + m, 1));
    }
    groups.push(group(alloc::vec![0, 1], &beta_swaps));
    groups.push(group(
        alloc::vec![2, 3, mcol, mcol + 1],
        &[(f_off, 2, m_off + mrow, mcol), (f_off, 3, m_off + mrow, mcol + 1)],
    ));

    // Step 2b, clean new bindings (no dependence on the fold eval point):
    // DEEP z (cols 10,11) == the transcript's squeezed out-of-domain point.
    groups.push(group(
        alloc::vec![0, 10, 11],
        &[(z_op * l, 0, d_off + 1, 10), ((z_op + 1) * l, 0, d_off + 1, 11)],
    ));
    // DEEP batched result (acc after the last term, cols 2,3) == the authenticated
    // DEEP value (opening 1 of the membership batch).
    let (dr, dcol) = (m_off + ocells[1].0, ocells[1].1);
    groups.push(group(
        alloc::vec![2, 3, dcol, dcol + 1],
        &[(d_off + n_terms, 2, dr, dcol), (d_off + n_terms, 3, dr, dcol + 1)],
    ));
    // DEEP composition value (the last term's val, cols 6,7) == the authenticated
    // composition opening (opening 2 of the batch).
    let (cr, ccol) = (m_off + ocells[2].0, ocells[2].1);
    groups.push(group(
        alloc::vec![6, 7, ccol, ccol + 1],
        &[(d_off + n_terms - 1, 6, cr, ccol), (d_off + n_terms - 1, 7, cr, ccol + 1)],
    ));

    // The core of the gap closure: every inner trace value feeding the DEEP batch
    // (cols 6, once per window row) is bound to its authenticated trace-column
    // opening (openings 3.. of the batch), so no opened value is trusted. Each is a
    // cycle over the window copies plus the leaf.
    let width_inner = ocells.len() - 3;
    let window_inner = (n_terms - 1) / width_inner;
    std::println!("inner width {}, window {}", width_inner, window_inner);
    for c in 0..width_inner {
        let leaf_row = m_off + ocells[3 + c].0;
        let leaf_col = ocells[3 + c].1;
        let mut sw: Vec<(usize, usize, usize, usize)> = Vec::new();
        for k in 0..window_inner - 1 {
            let ra = d_off + k * width_inner + c;
            let rb = d_off + (k + 1) * width_inner + c;
            sw.push((ra, 6, rb, 6));
        }
        let rlast = d_off + (window_inner - 1) * width_inner + c;
        sw.push((rlast, 6, leaf_row, leaf_col));
        groups.push(group(alloc::vec![6, leaf_col], &sw));
        // The imaginary lane: DEEP val.c1 (col 7) == leaf lane 1, both zero for a
        // base-field trace value, so the value cannot smuggle an extension part.
        let mut sw1: Vec<(usize, usize, usize, usize)> = Vec::new();
        for k in 0..window_inner - 1 {
            sw1.push((d_off + k * width_inner + c, 7, d_off + (k + 1) * width_inner + c, 7));
        }
        sw1.push((rlast, 7, leaf_row, leaf_col + 1));
        groups.push(group(alloc::vec![7, leaf_col + 1], &sw1));
    }

    // Root bindings: each opening's authenticated root (region-5 checkpoint row,
    // cols 0..RATE) == the transcript-absorbed root (region 0/3 inject column 8), so
    // every authentication is against the committed commitment, not a prover-chosen
    // one. Openings 0,1 (FRI leaf, deep) share fri.roots[0] absorbed in the FRI
    // transcript; opening 2 (comp) is comp_root and 3.. are trace_roots, in the
    // STARK transcript.
    let rate = 4usize;
    let ntr = proof.trace_roots.len();
    let ncoeff2 = ci.coeffs.len() * 2;
    for o in 0..ocells.len() {
        let cp_row = m_off + ocells[o].0 + depth * l;
        let mut sw: Vec<(usize, usize, usize, usize)> = Vec::new();
        for j in 0..rate {
            let arow = if o <= 1 {
                ft_off + j * l
            } else if o == 2 {
                (pub_len + ntr * rate + ncoeff2 + j) * l
            } else {
                (pub_len + (o - 3) * rate + j) * l
            };
            sw.push((cp_row, j, arow, 8));
        }
        groups.push(group(alloc::vec![0, 1, 2, 3, 8], &sw));
    }

    // DEEP coefficients: each term's batching coefficient (cols 12,13) == the
    // corresponding deep-coefficient squeezed in the transcript, so the DEEP batch
    // uses the verifier's coefficients, not the prover's.
    for i in 0..n_terms {
        let op = deep_coeff_op + 2 * i;
        groups.push(group(
            alloc::vec![0, 12, 13],
            &[(op * l, 0, d_off + i, 12), ((op + 1) * l, 0, d_off + i, 13)],
        ));
    }

    // The DEEP claims == the composition frame == the transcript-absorbed ood frame.
    // Each of the six trace-term claims is one out-of-domain trace value, cycled
    // across the three regions so the DEEP check, the composition, and the transcript
    // all agree on the same frame. A cycle over cells that must be equal.
    let cycle = |cells: &[(usize, usize)]| -> GpGroup {
        let mut wcols: Vec<usize> = cells.iter().map(|c| c.1).collect();
        wcols.sort_unstable();
        wcols.dedup();
        let mut swaps: Vec<(usize, usize, usize, usize)> = Vec::new();
        for w in 0..cells.len() - 1 {
            swaps.push((cells[w].0, cells[w].1, cells[w + 1].0, cells[w + 1].1));
        }
        group(wcols, &swaps)
    };
    for i in 0..6 {
        let ood_c0 = z_op + 2 + 2 * i;
        groups.push(cycle(&[(c_off, 2 * i), (d_off + i, 8), (ood_c0 * l, 8)]));
        groups.push(cycle(&[(c_off, 2 * i + 1), (d_off + i, 9), ((ood_c0 + 1) * l, 8)]));
    }

    let wired = WiredMultiExt::new(regions, groups);
    let witness = wired.trace(&[ttrace, ctrace, dtrace, fttrace, ftrace, mtrace]);
    std::println!(
        "gapclosed assembly: trace_width {}, log_trace_len {}, degree {}, groups {}",
        wired.trace_width(),
        wired.log_trace_len(),
        wired.constraint_degree(),
        wired.num_transition()
    );
    let wproof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(stark_verify_ext(&wired, &wproof, 32, 8), "the gap-closed assembly did not verify");
}

#[test]
fn a_poseidon_committed_stark_holds_at_deployment_blowup() {
    use crate::crypto::stark::air::{
        stark_prove_poseidon_ext, stark_verify_poseidon_ext, Squaring,
    };
    // rate 1/16 (extra_blowup_bits = 3): the inner proof at deployment soundness.
    let seed = Fp::from_u64(5);
    let air = Squaring { log_t: 4, seed };
    let trace = squaring_trace(4, seed);
    let h = hasher();
    let proof = stark_prove_poseidon_ext(&air, &trace, 32, 16, 3, &h);
    assert!(
        stark_verify_poseidon_ext(&air, &proof, 32, 16, 3, &h),
        "an honest deployment-soundness Poseidon STARK was rejected"
    );
}

#[test]
fn a_low_degree_poseidon_ext_codeword_verifies() {
    let (log_n, log_blowup) = (10u32, 1u32);
    let shift = Fp::from_u64(7);
    let d = 1usize << (log_n - log_blowup);
    let cw = low_degree_ext(log_n, d, shift, 0xABCD_1234);
    let h = hasher();
    let proof = fri_prove_poseidon_ext(&cw, shift, log_blowup, 32, 8, &h);
    assert!(
        fri_verify_poseidon_ext(&proof, shift, log_n, log_blowup, 32, 8, &h),
        "an honest low-degree Poseidon extension codeword was rejected"
    );
}

#[test]
fn a_high_degree_poseidon_ext_codeword_is_rejected() {
    let (log_n, log_blowup) = (10u32, 1u32);
    let shift = Fp::from_u64(7);
    // Degree equal to the domain size: not low degree for a rate-1/2 test.
    let cw = low_degree_ext(log_n, 1usize << log_n, shift, 0x9999);
    let h = hasher();
    let proof = fri_prove_poseidon_ext(&cw, shift, log_blowup, 32, 8, &h);
    assert!(
        !fri_verify_poseidon_ext(&proof, shift, log_n, log_blowup, 32, 8, &h),
        "a high-degree Poseidon extension codeword verified"
    );
}

#[test]
fn a_tampered_final_layer_is_rejected() {
    let (log_n, log_blowup) = (10u32, 1u32);
    let shift = Fp::from_u64(7);
    let d = 1usize << (log_n - log_blowup);
    let cw = low_degree_ext(log_n, d, shift, 0x5151);
    let h = hasher();
    let mut proof = fri_prove_poseidon_ext(&cw, shift, log_blowup, 32, 8, &h);
    proof.final_layer[0] = proof.final_layer[0] + Fp2::from_base(Fp::from_u64(1));
    assert!(
        !fri_verify_poseidon_ext(&proof, shift, log_n, log_blowup, 32, 8, &h),
        "a tampered final layer verified"
    );
}
