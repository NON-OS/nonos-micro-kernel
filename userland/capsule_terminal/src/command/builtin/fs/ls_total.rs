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

//! The `-l` header line. Real `ls` totals disk blocks; this system reports no
//! block count, so the total is a byte sum and the unit is left unclaimed
//! rather than mislabelled.

use alloc::vec::Vec;

use super::ls_long::Row;
use super::ls_num::{decimal, human_size};

pub fn total_line(rows: &[Row], human: bool) -> Vec<u8> {
    let sum = rows.iter().fold(0u64, |acc, r| acc.saturating_add(r.size));
    let mut line = Vec::from(&b"total "[..]);
    line.extend_from_slice(&if human { human_size(sum) } else { decimal(sum) });
    line
}
