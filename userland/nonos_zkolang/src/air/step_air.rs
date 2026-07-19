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

//! The AIR value: the padded trace length, the program's public wiring, and the
//! boundary values it binds. The methods that build and evaluate it live in the
//! sibling files; this file is just the state they share.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::wiring::WireRow;

/// The step AIR over a trace of `2^log_t` rows, carrying the public data-flow
/// wiring of the program it proves and the public input and output values it
/// binds.
pub struct StepAir {
    pub(super) log_t: u32,
    pub(super) wiring: Vec<WireRow>,
    /// Boundary triples binding public inputs and outputs: (column, row, value).
    pub(super) public_bindings: Vec<(usize, usize, Fp)>,
}
