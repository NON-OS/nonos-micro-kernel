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

//! FRI committed with Poseidon instead of BLAKE3. The commitments and the
//! transcript are algebraic, so a proof made here can be verified by an AIR:
//! the prerequisite for a recursive STARK. It is otherwise the same protocol.

mod fold;
mod prove;
mod types;
mod verify;

pub use prove::fri_prove;
pub use types::{FriProof, LayerOpening, QueryProof};
pub use verify::fri_verify;
