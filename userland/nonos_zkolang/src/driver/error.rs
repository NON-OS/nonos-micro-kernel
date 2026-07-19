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

//! Why a proving run did not complete. Each variant names the stage that failed
//! and carries that stage's own error, so a caller can tell a false claim (which
//! is `Execute`) from a malformed program (which is `Compile`).

use crate::air::BuildError;
use crate::lang::CompileError;
use crate::vm::ProveError;

/// The reasons a proving run stops short of a report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunError {
    /// The source did not compile.
    Compile(CompileError),
    /// The program ran but its witness violated a constraint (a failed assert, an
    /// inverse of zero), so there is no trace to prove. This is the honest result
    /// for a program whose claim is false.
    Execute(ProveError),
    /// The executed trace could not be laid out for the AIR.
    Layout(BuildError),
    /// The program needs more steps than the driver will size a trace to.
    ProgramTooLong { steps: usize },
}
