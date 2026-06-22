// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::pack_dirs::pack_dirs;

pub fn serialize(
    c: &[u8; 32],
    a: &[u8; 32],
    zx: &[u8; 32],
    zr: &[u8; 32],
    siblings: &[[u8; 32]],
    dirs: &[u8],
) -> Vec<u8> {
    let packed_dirs = pack_dirs(dirs);
    let mut out = Vec::with_capacity(129 + siblings.len() * 32 + packed_dirs.len());
    out.extend_from_slice(c);
    out.extend_from_slice(a);
    out.extend_from_slice(zx);
    out.extend_from_slice(zr);
    out.push(siblings.len() as u8);
    for sibling in siblings {
        out.extend_from_slice(sibling);
    }
    out.extend_from_slice(&packed_dirs);
    out
}
