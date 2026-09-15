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

//! The kernel's directory-record layout, assembled for the host.
//!
//! These modules are the shipping kernel source, included by `#[path]`. They
//! touch no block device, no cipher and no allocator, so the arithmetic that
//! decides which bytes are a file's name and which are its address runs here
//! exactly as it runs in ring 0.

#[path = "../../../../src/fs/blockfs/dir_block.rs"]
pub mod dir_block;
#[path = "../../../../src/fs/blockfs/dir_block_header.rs"]
pub mod dir_block_header;
#[path = "../../../../src/fs/blockfs/dir_consts.rs"]
pub mod dir_consts;
#[path = "../../../../src/fs/blockfs/dir_name_match.rs"]
pub mod dir_name_match;
#[path = "../../../../src/fs/blockfs/read_u32.rs"]
pub mod read_u32;
#[path = "../../../../src/fs/blockfs/read_u64.rs"]
pub mod read_u64;
#[path = "../../../../src/fs/blockfs/write_u32.rs"]
pub mod write_u32;
#[path = "../../../../src/fs/blockfs/write_u64.rs"]
pub mod write_u64;

pub mod surface;

#[cfg(test)]
mod tests;
