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

/// Penalty for a single row or column: runs of five or more same-colour
/// modules score 3, plus 1 for each module past the fifth.
pub(super) fn run_penalty(line: impl Iterator<Item = bool>) -> u32 {
    let mut score = 0u32;
    let mut prev = None;
    let mut run = 0u32;
    for v in line {
        if Some(v) == prev {
            run += 1;
        } else {
            if run >= 5 {
                score += 3 + (run - 5);
            }
            prev = Some(v);
            run = 1;
        }
    }
    if run >= 5 {
        score += 3 + (run - 5);
    }
    score
}
