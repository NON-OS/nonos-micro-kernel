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

use super::finder_penalty::finder_penalty;
use super::run_penalty::run_penalty;
use crate::matrix::Matrix;

/// Total penalty for the masked matrix across the four ISO/IEC 18004 rules;
/// lower reads more reliably.
pub(crate) fn penalty(m: &Matrix) -> u32 {
    let n = m.n;
    let mut score = 0u32;

    for y in 0..n {
        score += run_penalty((0..n).map(|x| m.get(x, y)));
    }
    for x in 0..n {
        score += run_penalty((0..n).map(|y| m.get(x, y)));
    }

    for y in 0..n - 1 {
        for x in 0..n - 1 {
            let a = m.get(x, y);
            if a == m.get(x + 1, y) && a == m.get(x, y + 1) && a == m.get(x + 1, y + 1) {
                score += 3;
            }
        }
    }

    for y in 0..n {
        score += finder_penalty(n, |x| m.get(x, y));
    }
    for x in 0..n {
        score += finder_penalty(n, |y| m.get(x, y));
    }

    let dark = m.modules.iter().filter(|&&d| d).count();
    let total = (n * n) as i32;
    let percent = (dark as i32 * 100) / total;
    let step = (percent - 50).abs() / 5;
    score += (step * 10) as u32;

    score
}
