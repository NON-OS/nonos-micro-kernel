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

//! The one-call driver: source and inputs in, a proven-and-verified report out.
//! This is the whole pipeline behind one function, the surface a shell command or
//! a capsule calls. Nothing here panics; every failure along the way is a typed
//! `RunError`.
//!
//! The pieces are split by concern: the report a run returns, the error it can
//! fail with, the prove pipeline over a program, and the source wrappers that
//! compile first.

mod error;
mod prove;
mod report;
mod source;

pub use error::RunError;
pub use prove::prove_program;
pub use report::Report;
pub use source::{prove_source, prove_source_with_inputs, prove_source_with_witness};

// The verifier-key helper sizes the trace the same way the prover does.
pub(crate) use prove::choose_log_t;
