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

//! The per-program verifier key: the object a proving market registers against a
//! program commitment so the on-chain statement binds the program's wiring, not
//! just its bytes.
//!
//! The step AIR's periodic columns are the program's data-flow wiring, so the
//! recursion's preprocessed periodic root must be tied to the program commitment;
//! otherwise a prover could satisfy one program's transition constraints under
//! another program's routing. The wiring is a deterministic public function of the
//! committed opcode list, so the binding is a public, recomputable relation, not a
//! trusted input. This module computes it.
//!
//! `periodic_root` goes through the same `nonos-stark` function the preprocessed
//! prover commits its periodic tree with, so a registered root and a proof's
//! committed root are the same object by construction. `verifier_key` hashes the
//! program's structure descriptor together with that root, with keccak256, the same
//! hash the market's Merkle authentication and the on-chain contract use, so a
//! challenger recomputes both halves from the committed opcode list. The
//! `wiring_version` byte fronts the descriptor, so a future change to the wiring
//! derivation is a new key, never a silent re-registration.

use alloc::vec::Vec;

use nonos_stark::air::{periodic_root as air_periodic_root, Air};
use nonos_stark::hash::keccak256;

use crate::air::{BuildError, StepAir, TRACE_WIDTH};
use crate::commit;
use crate::driver::choose_log_t;
use crate::isa::Op;

// The version of the wiring derivation and this descriptor's byte layout. Bump it
// if `WireRow::of`, `serialize`, or the field order below ever changes, so a
// derivation change forces a new key rather than colliding with an old one.
const WIRING_VERSION: u8 = 1;

/// The FRI blowup every zkolang key is registered at. The recursion's inner proof
/// and the on-chain contract fix this rate immutably, so a key registered at any
/// other rate could never match a real proof and a challenger would reject it. Use
/// this when deriving a market key rather than passing a rate literal; the rate is
/// part of the key, so the one place it is written is the one place it can drift.
pub const REGISTRATION_RATE: u32 = 3;

/// Why a verifier key could not be derived.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyError {
    /// The program has no reachable halt, so its trace length is undefined.
    NoHalt,
    /// The program needs a trace longer than the driver will size to.
    ProgramTooLong,
}

// The trace length before padding: the first halt's position plus one, matching
// how the VM stops at halt.
fn step_count(program: &[Op]) -> Option<usize> {
    program.iter().position(|op| matches!(op, Op::Halt)).map(|i| i + 1)
}

// Build the wiring-only AIR for a program, sizing the trace exactly as the driver
// does so the periodic columns match a real proof's.
fn air_for(program: &[Op]) -> Result<StepAir, KeyError> {
    let steps = step_count(program).ok_or(KeyError::NoHalt)?;
    let log_t = choose_log_t(steps).ok_or(KeyError::ProgramTooLong)?;
    StepAir::for_key(program, log_t).map_err(|e| match e {
        BuildError::NoHalt => KeyError::NoHalt,
        _ => KeyError::ProgramTooLong,
    })
}

/// The preprocessed-periodic root of a program's step AIR at the given FRI rate.
/// This is the exact tree the preprocessed prover commits, so a registered root and
/// a proof's committed root cannot diverge. `extra_blowup_bits` must match the rate
/// the recursion's inner proof uses.
pub fn periodic_root(program: &[Op], extra_blowup_bits: u32) -> Result<[u8; 32], KeyError> {
    Ok(air_periodic_root(&air_for(program)?, extra_blowup_bits))
}

/// The per-program verifier key: `keccak256(descriptor ‖ periodic_root)`. The
/// descriptor binds the wiring version, the exact program (through its 32-byte
/// commitment, which commits the whole opcode list and so every boundary and
/// wiring position), the padded trace length, the trace width, and the FRI rate.
/// A market registers `programCommit -> verifier_key`; a challenger recomputes both
/// halves from the committed opcode list and rejects a wrong entry.
pub fn verifier_key(program: &[Op], extra_blowup_bits: u32) -> Result<[u8; 32], KeyError> {
    let air = air_for(program)?;
    let root = air_periodic_root(&air, extra_blowup_bits);
    let mut buf: Vec<u8> = Vec::new();
    buf.push(WIRING_VERSION);
    buf.extend_from_slice(&commit::commit(program));
    buf.extend_from_slice(&air.log_trace_len().to_le_bytes());
    buf.extend_from_slice(&(TRACE_WIDTH as u32).to_le_bytes());
    buf.extend_from_slice(&extra_blowup_bits.to_le_bytes());
    buf.extend_from_slice(&root);
    Ok(keccak256(&buf))
}

/// The verifier key at the market's fixed registration rate. This is the value a
/// NOX proving market stores against a program commitment, so callers register and
/// challenge through this rather than choosing a rate at the call site.
pub fn registration_key(program: &[Op]) -> Result<[u8; 32], KeyError> {
    verifier_key(program, REGISTRATION_RATE)
}

/// The preprocessed-periodic root at the market's fixed registration rate, the tree
/// a conforming proof commits its periodic columns with.
pub fn registration_root(program: &[Op]) -> Result<[u8; 32], KeyError> {
    periodic_root(program, REGISTRATION_RATE)
}
