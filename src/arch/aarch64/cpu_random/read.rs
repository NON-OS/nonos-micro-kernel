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

//! RNDR and RNDRRS.
//!
//! Both are read as system registers and both report failure the same way: the
//! register reads as zero and PSTATE.NZCV is set to 0b0100, so the Z flag is
//! what distinguishes a genuine zero draw from a refusal. Neither asm block may
//! claim `preserves_flags`.
//!
//! They are encoded by hand as `S3_3_C2_C4_x` rather than by name so the
//! assembler accepts them without the build enabling `+rand` globally; the
//! encodings are the ones the ARM ARM assigns to RNDR and RNDRRS.

const RNDR_ATTEMPTS: u32 = 10;
const RNDRRS_ATTEMPTS: u32 = 100;

/// One attempt at the named FEAT_RNG register, yielding `Option<u64>` that is
/// `Some` only when the core reported the draw valid.
macro_rules! draw {
    ($reg:literal) => {{
        let value: u64;
        let ok: u64;
        // SAFETY: the enclosing function's caller guarantees FEAT_RNG is
        // present, so the register is defined and readable at EL1. The read
        // writes only the two output operands plus NZCV, which the absence of
        // `preserves_flags` declares. It touches no memory and needs no stack.
        unsafe {
            core::arch::asm!(
                concat!("mrs {value}, ", $reg),
                "cset {ok}, ne",
                value = out(reg) value,
                ok = out(reg) ok,
                options(nomem, nostack),
            );
        }
        if ok != 0 {
            Some(value)
        } else {
            None
        }
    }};
}

/// Draw from the conditioned DRBG.
///
/// # Safety
///
/// The core must implement FEAT_RNG; the system register is undefined
/// otherwise. Callers go through `arch::cpu_random`, which checks first.
pub unsafe fn rndr_u64() -> Option<u64> {
    for _ in 0..RNDR_ATTEMPTS {
        if let Some(value) = draw!("S3_3_C2_C4_0") {
            return Some(value);
        }
        core::hint::spin_loop();
    }
    None
}

/// Draw from the entropy source, forcing a reseed of the conditioner first.
///
/// # Safety
///
/// The core must implement FEAT_RNG; the system register is undefined
/// otherwise. Callers go through `arch::cpu_random`, which checks first.
pub unsafe fn rndrrs_u64() -> Option<u64> {
    for _ in 0..RNDRRS_ATTEMPTS {
        if let Some(value) = draw!("S3_3_C2_C4_1") {
            return Some(value);
        }
        core::hint::spin_loop();
    }
    None
}
