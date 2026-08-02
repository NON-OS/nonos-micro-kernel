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

//! Checking a tree read off disk is in the order it claims.

use core::cmp::Ordering;

use super::compare::compare;
use super::entry::TreeEntry;

/// Whether entries are in the required order with no duplicate name, which is
/// what a well-formed tree must satisfy. A tree failing this would re-encode
/// to a different id than the one it was read from.
pub(super) fn is_sorted_and_unique(entries: &[TreeEntry]) -> bool {
    entries.windows(2).all(|w| compare(&w[0], &w[1]) == Ordering::Less)
}
