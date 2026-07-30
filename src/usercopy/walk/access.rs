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

//! Access-aware translation. The only translation entry points
//! callable from the rest of the usercopy tree. Every byte transfer
//! window asks for `translate_read` (mapped + USER) or
//! `translate_write` (mapped + USER + WRITABLE) and never the
//! generic walker — there is no way to get a leaf without checking
//! permission first.

use super::leaf::UserLeaf;
use super::levels::walk_to_leaf;
use crate::arch::paging::descriptor;
use crate::usercopy::error::UsercopyError;

pub(crate) fn translate_read(va: u64) -> Result<UserLeaf, UsercopyError> {
    let leaf = walk_to_leaf(va)?;
    if !descriptor::is_user(leaf.entry) {
        return Err(UsercopyError::PageNotUser);
    }
    Ok(leaf)
}

pub(crate) fn translate_write(va: u64) -> Result<UserLeaf, UsercopyError> {
    let leaf = walk_to_leaf(va)?;
    if !descriptor::is_user(leaf.entry) {
        return Err(UsercopyError::PageNotUser);
    }
    if !descriptor::is_writable(leaf.entry) {
        return Err(UsercopyError::PageNotWritable);
    }
    Ok(leaf)
}
