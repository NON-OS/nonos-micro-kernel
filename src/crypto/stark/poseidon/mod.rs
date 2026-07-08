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

//! Poseidon over Goldilocks: the permutation, its published parameters, and a
//! field sponge hash built on it. Used as the algebraic hash inside STARK
//! constraints, where an arithmetic-friendly permutation lets a proof reason
//! about hashing without a bit-level circuit.

pub mod constants;
pub mod permutation;
pub mod sponge;

pub use constants::{FULL_ROUNDS, N_ROUNDS, PARTIAL_ROUNDS, WIDTH};
pub use permutation::permute;
pub use sponge::{compress, hash, DIGEST, RATE};
