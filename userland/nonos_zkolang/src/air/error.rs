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

//! Why a program or VM trace could not be laid out for the step AIR.

/// The reasons `compile` or `build_trace` refuse a program or trace.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildError {
    /// The program has no reachable halt, so its length is undefined.
    NoHalt,
    /// The run is longer than the requested power-of-two trace length.
    TooLong { rows: usize, cap: usize },
    /// An `Out` names a public output index with no supplied value.
    MissingPublicOutput { idx: u16 },
}
