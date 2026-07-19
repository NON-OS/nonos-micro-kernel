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

//! The program commitment: a canonical byte encoding of a compiled program and a
//! stable hash over it. On chain a proving job is posted against `commit`, the
//! 32-byte digest, so buyer and verifier agree on one exact program. Inside a
//! proof the same commitment enters as `commit_limbs`, four field elements bound
//! into the transcript, so the proof is tied to the program it claims to run.
//!
//! The encoding lives in `serialize`, the hashing in `digest`, so the shape of the
//! bytes and the shape of the commitment are each stated in one place.

mod digest;
mod serialize;

pub use digest::{commit, commit_limbs};
pub use serialize::serialize;
