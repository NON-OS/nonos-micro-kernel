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

//! A table is however many adjacent blocks parse as rows. Nothing marks where
//! one starts, so the extent is recovered from the neighbours each time and can
//! never disagree with the text the user is looking at.

use crate::doc::document::Doc;
use crate::doc::table::syntax::is_row;

pub fn is_table_block(doc: &Doc, i: usize) -> bool {
    doc.blocks.get(i).map(|b| is_row(b.as_str())).unwrap_or(false)
}

pub fn run_of(doc: &Doc, i: usize) -> Option<(usize, usize)> {
    if !is_table_block(doc, i) {
        return None;
    }
    let mut start = i;
    while start > 0 && is_table_block(doc, start - 1) {
        start -= 1;
    }
    let mut end = i + 1;
    while is_table_block(doc, end) {
        end += 1;
    }
    Some((start, end))
}
