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

//! The zkolang step AIR: the branchless computational core with register binding
//! and public input and output binding.
//!
//! This AIR proves, for a trace laid out one VM step per row:
//!
//!   1. every row carries exactly one opcode (the selectors are boolean and
//!      sum to one), so no row is ambiguous or opcode-free;
//!   2. every row's result equals the operation its selector names: field add,
//!      subtract, multiply, and invert; an equality test that yields a clean bit;
//!      a conditional select; and the two constraint opcodes, a boolean check and
//!      a zero assertion, that let a program state a fact the proof must uphold;
//!   3. every operand a row reads is the live value of the register it names, and
//!      every register carries its value forward unchanged until the row that
//!      writes it, at which point it takes that row's result;
//!   4. the input a row reads and the output a row exposes match the public
//!      values the proof commits to, so the proof attests a statement about
//!      public data, not merely that some self-contained run existed;
//!   5. the rows are clock-ordered, the counter rising by one each step.
//!
//! Points three and four together are what make a proof economically meaningful:
//! it says a specific public function of specific public inputs produced specific
//! public outputs. Register indices are compile-time, so the data flow is a public
//! circuit carried as periodic one-hot columns, and both reads and writes are
//! linear in the trace. Public inputs and outputs are bound by boundary
//! constraints: the verifier reconstructs the same AIR from the same public
//! program and public values, so a prover cannot substitute either.
//!
//! The module is split by concern: the column layout and constants, the error
//! type, the per-row wiring, the AIR struct, its compile and build-trace steps,
//! the transition written once over the `Felt` abstraction, and the trait impls
//! that hand the transition to the framework over both the base and the extension
//! field.

mod air_impl;
mod build_trace;
mod compile;
mod error;
mod layout;
mod step_air;
mod transition;
mod wiring;

pub use error::BuildError;
pub use layout::TRACE_WIDTH;
pub use step_air::StepAir;
