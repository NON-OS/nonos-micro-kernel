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

//! A Merkle commitment whose node hash is the Poseidon permutation rather than
//! BLAKE3. Its point is not speed but arithmetization: every node is a fixed
//! sequence of field operations, so a path check can be expressed as AIR
//! constraints and proven inside another STARK. That is the commitment a
//! recursive verifier needs, since a bitwise hash like BLAKE3 cannot be proven
//! cheaply. Digests are `RATE` field elements; leaves are already digests.

mod pack_base;
mod pack_ext;
mod tree;
mod verify;

pub use pack_base::pack_base;
pub use pack_ext::pack_ext;
pub use tree::PoseidonMerkleTree;
pub use verify::verify_path;
