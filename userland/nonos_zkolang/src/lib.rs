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

//! zkolang verifiable-compute VM (Phase 1 core).
//!
//! A register and memory VM over the Goldilocks field whose execution trace the
//! NONOS in-kernel transparent STARK proves. This crate is the VM, instruction
//! set, trace model, and the step AIR the prover reads. The zkolang language
//! front-end and the NOX proving-fee rail build on top of it.
//!
//! The field and the Poseidon permutation are the kernel STARK's, taken from the
//! `nonos-stark` crate. A VM register holds an `Fp`, the same scalar the STARK
//! commits, so a run and its proof are the same object with no field translation
//! between them and no second definition of the modulus anywhere in the tree.

#![no_std]

extern crate alloc;

mod air;
mod commit;
mod driver;
mod isa;
mod lang;
mod nox;
mod trace;
mod vkey;
mod vm;

pub use air::{BuildError, StepAir, TRACE_WIDTH};
pub use commit::{commit, commit_limbs, serialize};
pub use driver::{
    prove_program, prove_source, prove_source_with_inputs, prove_source_with_witness, Report,
    RunError,
};
pub use isa::{Op, Program, REGS};
pub use lang::{compile_source, CompileError};
pub use nox::{quote, Quote, MICRONOX_PER_NOX};
pub use trace::{OpTag, Row, Trace};
pub use vkey::{
    periodic_root, registration_key, registration_root, verifier_key, KeyError, REGISTRATION_RATE,
};
pub use vm::{ProveError, Vm};
