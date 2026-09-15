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

//! The included kernel functions, published as this crate's API.
//!
//! The kernel declares these `pub(super)`, which is right there and leaves
//! them looking dead here, where the only caller is a test module. Rather
//! than silencing the lint, each one is re-published so the non-test build
//! also compiles them and any future dead-code warning in this crate is a
//! real one.

use super::{dir_block, dir_block_header};

pub fn is_record(block: &[u8]) -> bool {
    dir_block_header::is_record(block)
}
pub fn count(block: &[u8]) -> usize {
    dir_block_header::count(block)
}
pub fn set_count(block: &mut [u8], n: usize) {
    dir_block_header::set_count(block, n)
}
pub fn has_room(block: &[u8]) -> bool {
    dir_block_header::has_room(block)
}
pub fn next(block: &[u8]) -> u64 {
    dir_block_header::next(block)
}
pub fn set_next(block: &mut [u8], lba: u64) {
    dir_block_header::set_next(block, lba)
}
pub fn name(block: &[u8], i: usize) -> &[u8] {
    dir_block::name(block, i)
}
pub fn entry_lba(block: &[u8], i: usize) -> u64 {
    dir_block::lba(block, i)
}
pub fn find(block: &[u8], wanted: &[u8]) -> Option<usize> {
    dir_block::find(block, wanted)
}
