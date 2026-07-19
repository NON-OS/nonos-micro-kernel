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

//! The zkolang VM executor. It runs a compiled program on public and private
//! inputs and emits the execution trace the STARK proves. It never panics: a
//! malformed program is a typed error, and a violated constraint (a failed
//! assert, an inverse of zero, a non-boolean selector) is reported as
//! `Unprovable`, the honest result, because such a trace has no proof.
//!
//! The executor is split so each file carries one concern: the error type, the
//! machine state and its register access, the run loop, and the per-opcode step.

mod error;
mod machine;
mod run;
mod step;

pub use error::ProveError;
pub use machine::Vm;
