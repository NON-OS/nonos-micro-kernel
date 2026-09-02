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

/// The kernel row's text. VERSION carries the trailing newline every text file
/// ends with, which the mono face draws as a replacement box, so it is trimmed
/// here rather than at each call site.
pub fn kernel_line(out: &mut [u8]) -> usize {
    let label = b"microkernel ";
    let ver = include_str!("../../../../VERSION").trim_end().as_bytes();
    let n = (label.len() + ver.len()).min(out.len());
    out[..label.len()].copy_from_slice(label);
    out[label.len()..n].copy_from_slice(&ver[..n - label.len()]);
    n
}
