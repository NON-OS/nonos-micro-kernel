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

pub fn gts_r4_anchor(spki_hash: &[u8; 32]) -> bool {
    const GTS_R4: [u8; 32] = [
        0x98, 0x47, 0xe5, 0x65, 0x3e, 0x5e, 0x9e, 0x84, 0x75, 0x16, 0xe5, 0xcb, 0x81, 0x86, 0x06,
        0xaa, 0x75, 0x44, 0xa1, 0x9b, 0xe6, 0x7f, 0xd7, 0x36, 0x6d, 0x50, 0x69, 0x88, 0xe8, 0xd8,
        0x43, 0x47,
    ];
    spki_hash == &GTS_R4
}
