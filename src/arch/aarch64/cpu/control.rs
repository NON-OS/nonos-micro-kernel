// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use core::arch::asm;

/// Put SCTLR_EL1 into the state the rest of bring-up expects.
///
/// Deliberately leaves M, C and I alone. Translation and the caches come on
/// together in `mmu::control::enable_mmu`, once TTBR and TCR are loaded and the
/// tables describe the image. Setting M here faults on the next instruction
/// fetch, before the vectors are installed to report it, so the machine hangs
/// with nothing on the console to say why.
///
/// SA0 keeps stack alignment checked at EL0. UCI lets EL0 issue the cache
/// maintenance it is permitted. WXN is cleared because write implying
/// execute-never belongs in the tables, stated per mapping, not as one global
/// switch that the page attributes then have to work around.
pub(super) fn configure_sctlr() {
    const SCTLR_SA0: u64 = 1 << 4;
    const SCTLR_WXN: u64 = 1 << 19;
    const SCTLR_UCI: u64 = 1 << 26;

    let mut sctlr: u64;
    // SAFETY: SCTLR_EL1 is readable and writable at EL1. None of the bits
    // touched here affect translation, which is still off.
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nostack));
    }
    sctlr |= SCTLR_SA0 | SCTLR_UCI;
    sctlr &= !SCTLR_WXN;
    unsafe {
        asm!("msr sctlr_el1, {}", in(reg) sctlr, options(nostack));
        asm!("isb", options(nostack));
    }
}

/// Let this exception level use the vector registers.
///
/// FPEN covers FP and Advanced SIMD. ZEN covers SVE, and it is a separate pair
/// of bits: a part that implements SVE traps every SVE instruction while ZEN is
/// clear, and reports it as an unknown-reason exception rather than as anything
/// naming the vector unit. That matters because the compiler reaches for SVE on
/// its own, in a `memcpy` or a struct move, so the trap arrives from ordinary
/// code with nothing to suggest why.
///
/// ZEN is only set where SVE is implemented. Writing it on a part without SVE
/// is architecturally a no-op, but reading the feature first keeps this honest
/// about what the machine actually has.
pub(super) fn enable_fp_simd() {
    use super::features::{has_feature, CpuFeature};

    const CPACR_FPEN: u64 = 3 << 20;
    const CPACR_ZEN: u64 = 3 << 16;

    // FPEN goes on first and unconditionally, before any call. Reading the
    // feature registers is ordinary Rust and the compiler is free to use a
    // vector register for it, so asking about SVE while FP is still trapped
    // faults inside the function whose job is to stop that happening.
    //
    // SAFETY: CPACR_EL1 is writable at EL1 and widening access cannot fault.
    // The ISB makes it visible before the next instruction, which may be the
    // first to use the registers.
    unsafe {
        asm!(
            "mrs {tmp}, cpacr_el1",
            "orr {tmp}, {tmp}, {bits}",
            "msr cpacr_el1, {tmp}",
            "isb",
            tmp = out(reg) _,
            bits = in(reg) CPACR_FPEN,
            options(nostack)
        );
    }

    if !has_feature(CpuFeature::Sve) {
        return;
    }
    // SAFETY: as above.
    unsafe {
        asm!(
            "mrs {tmp}, cpacr_el1",
            "orr {tmp}, {tmp}, {bits}",
            "msr cpacr_el1, {tmp}",
            "isb",
            tmp = out(reg) _,
            bits = in(reg) CPACR_ZEN,
            options(nostack)
        );
    }
}

pub(super) fn configure_cache() {
    unsafe {
        asm!("ic iallu", options(nostack));
        asm!("dsb ish", options(nostack));
        asm!("isb", options(nostack));
    }
}
