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

// The block-layer error type from capsule source. Only the error enum is
// included: the transport itself is syscall-bound, but its failure taxonomy is
// what the vfs handlers turn into errnos, so the proofs pin the real variants.

#[path = "../../../capsule_vfs/src/blk/error.rs"]
pub mod error;

/*
 * The table-of-contents patch that a same-length replacement writes, with the
 * header and toc modules it reads, so the offsets under test are the shipping
 * ones rather than numbers repeated here.
 */
pub mod store_patch;
