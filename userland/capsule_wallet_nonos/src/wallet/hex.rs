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

pub fn hex_addr(src: &[u8; 20], out: &mut [u8; 42]) {
    const H: &[u8; 16] = b"0123456789abcdef";
    out[0] = b'0';
    out[1] = b'x';
    for i in 0..20 {
        out[2 + i * 2] = H[(src[i] >> 4) as usize];
        out[3 + i * 2] = H[(src[i] & 0x0F) as usize];
    }
}

/// Short "0x1234...abcd" form: the first and last two address bytes with an
/// ellipsis between, for labels where the full address does not fit. The
/// buffer holds 2 prefix + 4 hex + 3-byte ellipsis + 4 hex = 13 bytes.
pub fn short_addr(src: &[u8; 20], out: &mut [u8; 13]) {
    const H: &[u8; 16] = b"0123456789abcdef";
    out[0] = b'0';
    out[1] = b'x';
    out[2] = H[(src[0] >> 4) as usize];
    out[3] = H[(src[0] & 0x0F) as usize];
    out[4] = H[(src[1] >> 4) as usize];
    out[5] = H[(src[1] & 0x0F) as usize];
    // U+2026 horizontal ellipsis.
    out[6] = 0xE2;
    out[7] = 0x80;
    out[8] = 0xA6;
    out[9] = H[(src[18] >> 4) as usize];
    out[10] = H[(src[18] & 0x0F) as usize];
    out[11] = H[(src[19] >> 4) as usize];
    out[12] = H[(src[19] & 0x0F) as usize];
}
