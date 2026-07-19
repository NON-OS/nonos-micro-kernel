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

//! Why a run produced no valid trace. Every variant is a legitimate outcome the
//! caller can inspect, never a panic.

/// The reasons the executor stops without a provable trace. `Unprovable` is not a
/// bug: it means the witness did not satisfy the program's constraints, which is
/// the honest result for a program whose claim is false.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProveError {
    /// A register index outside `0..REGS`.
    BadRegister(u8),
    /// An input index past the supplied input vector.
    BadInput(u16),
    /// The program ran its whole instruction list without a `Halt`.
    NoHalt,
    /// A constraint the trace must satisfy did not hold, at this step.
    Unprovable { step: u64 },
}