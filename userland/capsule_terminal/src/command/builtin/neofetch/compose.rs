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

//! Zips the emblem against the info rows into one column pair.

use alloc::vec::Vec;

use super::logo::LOGO_W;

/// Lay `left` beside `right`, padding the left column to `LOGO_W + gap`
/// columns and running to whichever side is longer.
///
/// The padding counts characters, not bytes: the emblem rows are box-drawing
/// glyphs, so a byte-based pad would indent the info column by three times
/// the intended amount. Trailing blanks are dropped so a row with nothing on
/// its right never carries invisible width into the scrollback.
pub fn two_column(left: &[&str], right: &[Vec<u8>], gap: usize) -> Vec<Vec<u8>> {
    let width = LOGO_W + gap;
    let rows = core::cmp::max(left.len(), right.len());
    let mut out = Vec::with_capacity(rows);
    for index in 0..rows {
        let mut line = Vec::with_capacity(width + 48);
        let mut cols = 0;
        if let Some(art) = left.get(index) {
            line.extend_from_slice(art.as_bytes());
            cols = art.chars().count();
        }
        while cols < width {
            line.push(b' ');
            cols += 1;
        }
        if let Some(info) = right.get(index) {
            line.extend_from_slice(info);
        }
        while line.last() == Some(&b' ') {
            line.pop();
        }
        out.push(line);
    }
    out
}
