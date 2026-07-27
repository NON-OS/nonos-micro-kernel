// NONOS Operating System (AGPL-3.0-or-later)
//! The recursive-vector artifact: the deployment-soundness proof of the
//! assembled verifier, with the batch publics' positions in the transcript
//! inject column.

use super::wire::serialize_pre;
use crate::crypto::stark::air::{Air, StarkProofExtPre, WIDTH};
use crate::recursion_assembly::build::Assembly;
use alloc::string::String;

pub(super) const NOTE: &str = "The full nine-region witness-mode recursion \
over a real Poseidon-committed join-split proof that absorbed the batch's \
K*11 publics. Every challenge is proven squeezed, every opened value \
authenticated, every evaluation point derived in-region, and the publics \
ride the transcript inject column (the FS input column) at (i*11+j)*l. The \
proof carries the preprocessed-periodic sidecar: after the base layout \
(spec/stark-serialization.md) comes u32 n_periodic, the n_periodic Fp2 \
claims at z (absorbed into the transcript right after the ood frame, before \
the DEEP coefficient draw, which is extended by one coefficient per \
periodic column), then per consistency query in order: n_periodic Fp row \
values and one path (u32 len + 32-byte digests) against the baked \
periodic_root. The DEEP value at each query includes the periodic quotient \
terms (P_j(x) - Pz_j)/(x - z). settleBatch: verify -> extract at the inject \
column, rows (i*11+j)*l -> gate. Representative publics in the frozen \
11-word order; the real batch swaps them, the layout unchanged.";

pub(super) fn emit(asm: &Assembly, wproof: &StarkProofExtPre, fri_log_blowup: u32, path: &str) {
    let words = 11usize;
    let l = asm.lay.l;
    let mut pubs = String::from("[");
    for (idx, p) in asm.publics.iter().enumerate() {
        if idx > 0 {
            pubs.push(',');
        }
        let (i, j) = (idx / words, idx % words);
        pubs.push_str(&alloc::format!("[{},{},{},\"{}\"]", i, j, (i * words + j) * l, p.value()));
    }
    pubs.push(']');

    let bytes = serialize_pre(wproof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"artifact\": \
         \"production-recursive-vector\",\n  \"note\": \"{}\",\n  \"l\": {}, \
         \"fs_input_column_index\": {}, \"k_intents\": {}, \"words_per_intent\": {},\n  \
         \"n_queries\": 32, \"grind_bits\": 16, \"extra_blowup_bits\": 3, \
         \"fri_log_blowup\": {}, \"trace_width\": {}, \"n_periodic\": {},\n  \
         \"publics\": {},\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        NOTE,
        l,
        WIDTH,
        asm.publics.len() / words,
        words,
        fri_log_blowup,
        asm.wired.trace_width(),
        wproof.periodic_z.len(),
        pubs,
        bytes.len(),
        crate::stark_selftest_gen::hex(&bytes)
    );
    std::fs::write(path, &json).expect("write vector");
    std::println!(
        "wrote production vector: {} publics, {} proof bytes, 9 regions",
        asm.publics.len(),
        bytes.len()
    );
}
