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

/// A 3-tap moving average over the sample ring, de-rotated from `head` into
/// `out` in one pass. The ends repeat their neighbour rather than fading to
/// zero, so a steady series stays flat instead of dipping at the edges. The
/// stored history is never touched; this smooths only what gets drawn.
pub fn smooth(src: &[u8], head: usize, out: &mut [u8]) -> usize {
    let n = src.len().min(out.len());
    if n == 0 {
        return 0;
    }
    let at = |i: usize| src[(head + i) % src.len()] as u32;
    for i in 0..n {
        let v = (at(i.saturating_sub(1)) + at(i) + at((i + 1).min(n - 1))) / 3;
        out[i] = v.min(100) as u8;
    }
    n
}
