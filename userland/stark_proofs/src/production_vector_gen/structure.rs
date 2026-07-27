// NONOS Operating System (AGPL-3.0-or-later)
//! The AIR-structure artifact: the region stack, the boundary set, the domain
//! parameters, and the baked preprocessed-periodic root the verifier holds as
//! a constant.

use super::json::hex32;
use crate::crypto::stark::air::Air;
use crate::recursion_assembly::build::Assembly;
use alloc::string::String;

pub(super) const NOTE: &str = "The witness-mode recursion AIR: nine regions \
(0 STARK transcript, 1 compose, 2 DEEP, 3 FRI transcript, 4 fold, 5 batched \
authentication, 6 DEEP-x index chain, 7 fold-point index chain, 8 periodic-z \
barycentric) stacked, the copy constraint split across independent \
grand-product columns so the degree stays at the region maximum. Every \
challenge is proven squeezed, every opened value authenticated, the DEEP x \
and the per-layer fold points derived in-region from the bound index bits, \
and the inner periodic values at z recomputed in-region from z. The periodic \
columns are structural: their eval-domain extension is committed once under \
the periodic wide-leaf domain and the root baked into the verifier. \
Deployment soundness is rate 1/16, 32 queries, 16 grind bits = 128-bit \
conjectured.";

#[allow(clippy::too_many_arguments)]
pub(super) fn emit(
    asm: &Assembly,
    off_heights: &[(usize, usize)],
    fri_log_blowup: u32,
    log_dn: u32,
    n_periodic: usize,
    region_transitions: usize,
    periodic_root: &[u8; 32],
    path: &str,
) {
    let wired = &asm.wired;
    let bnd = wired.boundary();
    let mut regs = String::from("[");
    for (ri, (o, hgt)) in off_heights.iter().enumerate() {
        if ri > 0 {
            regs.push(',');
        }
        regs.push_str(&alloc::format!("{{\"region\":{},\"offset\":{},\"height\":{}}}", ri, o, hgt));
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

    let json = alloc::format!(
        "{{\n  \"artifact\": \"production-air-structure\",\n  \"note\": \"{}\",\n  \
         \"log_trace_len\": {}, \"trace_width\": {}, \"constraint_degree\": {}, \
         \"num_transition\": {}, \"region_transitions\": {}, \"num_groups\": {},\n  \
         \"n_queries\": 32, \"grind_bits\": 16, \"extra_blowup_bits\": 3, \
         \"fri_log_blowup\": {}, \"log_eval_domain\": {},\n  \
         \"num_periodic_columns\": {}, \"num_boundaries\": {},\n  \
         \"periodic_root\": \"{}\",\n  \"regions\": {},\n  \"boundaries\": {}\n}}\n",
        NOTE,
        wired.log_trace_len(),
        wired.trace_width(),
        wired.constraint_degree(),
        wired.num_transition(),
        region_transitions,
        asm.n_groups,
        fri_log_blowup,
        log_dn,
        n_periodic,
        bnd.len(),
        hex32(periodic_root),
        regs,
        bstr
    );
    std::fs::write(path, &json).expect("write structure");
    std::println!(
        "wrote AIR structure: width {}, {} periodic, {} boundaries, {} groups, log_eval_domain {}",
        wired.trace_width(),
        n_periodic,
        bnd.len(),
        asm.n_groups,
        log_dn
    );
}
