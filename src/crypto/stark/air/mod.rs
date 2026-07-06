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

//! An end-to-end STARK over an algebraic intermediate representation. A trace is
//! interpolated and extended onto an evaluation coset, the constraints become a
//! composition polynomial, FRI proves that composition is low degree, and query
//! openings bind it to the committed trace. This closes the transparent,
//! post-quantum proof system: the verifier is proven against forgeries, not
//! assumed sound.

mod constraints;
mod prove;
mod types;
mod verify;

pub use prove::stark_prove;
pub use types::{StarkProof, StarkQuery};
pub use verify::stark_verify;
