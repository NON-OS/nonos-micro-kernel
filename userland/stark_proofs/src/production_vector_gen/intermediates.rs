// NONOS Operating System (AGPL-3.0-or-later)
//! The intermediates oracle: replay the outer verify on the emitted proof and
//! dump the known-good field elements each verifier stage must reproduce, so
//! the contracts side gates every stage against exact values, not only the
//! final accept.

use super::json::{bools, fp, fp2, fp2s, fps, usizes};
use crate::crypto::stark::air::{compose_ext, Air, AirExt, StarkProofExtPre};
use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poly::eval_lagrange_ext;
use crate::crypto::stark::transcript::Transcript;
use crate::recursion_assembly::build::Assembly;
use alloc::string::String;
use alloc::vec::Vec;

pub(super) fn emit(asm: &Assembly, pre: &StarkProofExtPre, path: &str) {
    let wproof = &pre.proof;
    let wired = &asm.wired;
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

    // The outer STARK transcript, exactly as the verifier draws it.
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
    // The periodic claims, absorbed before the DEEP draw; the coefficient
    // count gains one per periodic column.
    for value in &pre.periodic_z {
        ts.absorb_fp(value.c0);
        ts.absorb_fp(value.c1);
    }
    let n_periodic = pre.periodic_z.len();
    let deep_coeffs: Vec<Fp2> =
        (0..widthw * wsz + 1 + n_periodic).map(|_| ts.challenge_fp2()).collect();

    // The composition at z and its per-constraint breakdown. The claims must
    // equal this replay's own recomputation.
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
    assert_eq!(periodic_z, pre.periodic_z, "the sidecar claims diverge from the recompute");
    let comp_z = compose_ext(wired, gg, z, &wproof.ood_frame, &periodic_z, &coeffs);
    let transition_z = wired.transition_ext(&wproof.ood_frame, &periodic_z);

    // The FRI transcript: fold challenges then the FRI query positions.
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

    // The outer consistency query positions.
    ts.absorb_digest(&wproof.fri.roots[0]);
    let cons_qidx: Vec<usize> = (0..32).map(|_| ts.challenge_index(nn)).collect();

    let p0 = cons_qidx[0];
    let x0 = shift * omega.pow(p0 as u64);
    let q0 = &wproof.queries[0];
    let fq0 = &wproof.fri.queries[0];

    let mut layers = String::from("[");
    for (i, lyr) in fq0.layers.iter().enumerate() {
        if i > 0 {
            layers.push(',');
        }
        layers.push_str(&alloc::format!("{{\"a\":{},\"b\":{}}}", fp2(lyr.a), fp2(lyr.b)));
    }
    layers.push(']');

    // The inner-leaf ground truth for the fold-versus-leaf binding: the opened
    // cell column derives from the query's leaf-level direction, not a pin.
    let dirs: Vec<bool> = (0..asm.lay.depth).map(|lv| (asm.lay.i0 >> lv) & 1 == 1).collect();
    let (mrow, mcol) = asm.lay.ocells[0];

    let ntr = wired.num_transition();
    let json = alloc::format!(
        "{{\n  \"artifact\": \"production-intermediates\",\n  \"note\": \"Known-good field \
         elements from replaying the outer verify on the emitted vector. Gate each verifier \
         stage against these, not only the final accept. transition_z is the exact output of \
         the width-{tw} transition_ext at z: the first {region_tr} entries are the region \
         constraints (selector-weighted at z), the last {ngrp} are the grand-product terms; \
         your transition_ext must match this array elementwise. comp_z is the batched \
         composition your compose_ext must reproduce. periodic_z is the {nper}-column \
         periodic evaluation at z: it ships in the proof sidecar as claims, absorbed into \
         the transcript right after the ood frame, and deep_coeffs is extended to \
         width*window + 1 + num_periodic (trace terms, composition term, then one per \
         periodic column); the DEEP value at each query adds the periodic quotients \
         (P_j(x) - Pz_j)/(x - z) from the root-authenticated openings. \
         fri_query_indices come from the FRI \
         transcript, consistency_query_indices from the outer one. Fp as decimal string, \
         Fp2 as [c0,c1].\",\n  \"log_trace_len\": {ltl}, \"trace_width\": {tw}, \
         \"window_size\": {wsz}, \"log_eval_domain\": {ln}, \"num_transition\": {ntr}, \
         \"num_coeffs\": {nc}, \"num_periodic\": {nper},\n  \"z\": {z},\n  \"coeffs\": \
         {coeffs},\n  \"deep_coeffs\": {deepc},\n  \"betas\": {betas},\n  \
         \"fri_query_indices\": {friq},\n  \"consistency_query_indices\": {consq},\n  \
         \"comp_z\": {compz},\n  \"transition_z\": {trz},\n  \"periodic_z\": {perz},\n  \
         \"query0\": {{ \"index\": {p0}, \"x\": {x0}, \"trace_row\": {row}, \"comp\": \
         {qcomp}, \"deep\": {qdeep} }},\n  \"fri_query0\": {{ \"layers\": {layers}, \
         \"final_value\": {fv} }},\n  \"merkle_query0\": {{ \"note\": \"fold==leaf opened \
         cell. Derive opened_col from the leaf-level direction, do not pin it. \
         inner_leaf_index i0, directions per level (LSB first), opened_row/opened_col are \
         the ground truth.\", \"inner_leaf_index\": {i0}, \"directions\": {dirs}, \
         \"opened_row\": {mrow}, \"opened_col\": {mcol} }}\n}}\n",
        region_tr = ntr - asm.n_groups,
        ngrp = asm.n_groups,
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
        friq = usizes(&fri_qidx),
        consq = usizes(&cons_qidx),
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
        i0 = asm.lay.i0,
        dirs = bools(&dirs),
        mrow = mrow,
        mcol = mcol,
    );
    std::fs::write(path, &json).expect("write intermediates");
    std::println!(
        "wrote intermediates: z, {} coeffs, {} deep_coeffs, {} betas, comp_z, {} transition_z, {} periodic_z",
        coeffs.len(),
        deep_coeffs.len(),
        betas.len(),
        transition_z.len(),
        periodic_z.len()
    );
}
