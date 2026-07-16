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

//! Automatic table-layout column widths (CSS 2.1 section 17.5.2). Each column
//! has a min-content and a max-content width, taken across every cell in that
//! column; this distributes the available table width over them:
//!
//!   * when there is room for every column's max-content, columns take their
//!     max and the surplus is shared in proportion to those maxima;
//!   * when the room is between the totals, every column shrinks from its max
//!     toward its min in proportion to its own shrinkable range;
//!   * when there is not even room for the minima, columns take their min and
//!     the table overflows.
//!
//! The result always sums to the available width in the first two cases (the
//! rounding remainder lands on the last column) so cells tile the row exactly.
//! This is the piece a table formatter builds cell placement on; it is pure and
//! pinned by the browser proofs.

use alloc::vec;
use alloc::vec::Vec;

pub fn column_widths(max: &[i32], min: &[i32], avail: i32) -> Vec<i32> {
    let n = max.len();
    if n == 0 {
        return Vec::new();
    }
    let total_max: i64 = max.iter().map(|&w| w.max(0) as i64).sum();
    let total_min: i64 = min.iter().map(|&w| w.clamp(0, i32::MAX) as i64).sum();
    let avail = avail.max(0) as i64;
    let mut out = vec![0i32; n];

    if avail >= total_max {
        // Room to spare: each column its max, the surplus shared by max weight.
        let extra = avail - total_max;
        let mut assigned = 0i64;
        for (i, &m) in max.iter().enumerate() {
            let m = m.max(0) as i64;
            let add = if total_max > 0 { extra * m / total_max } else { extra / n as i64 };
            out[i] = (m + add) as i32;
            assigned += m + add;
        }
        out[n - 1] += (avail - assigned) as i32;
    } else if avail > total_min {
        // Between the totals: shrink each column across its own min..max range.
        let span = (total_max - total_min).max(1);
        let take = avail - total_min;
        let mut assigned = 0i64;
        for i in 0..n {
            let mn = min[i].max(0) as i64;
            let mx = (max[i].max(0) as i64).max(mn);
            let w = mn + (mx - mn) * take / span;
            out[i] = w as i32;
            assigned += w;
        }
        out[n - 1] += (avail - assigned) as i32;
    } else {
        // Not even room for the minima: take the min, let the table overflow.
        for (i, &mn) in min.iter().enumerate() {
            out[i] = mn.max(0);
        }
    }
    out
}
