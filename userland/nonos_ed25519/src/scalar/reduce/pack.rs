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

pub(crate) fn pack_output(a: &[i64; 24]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[0] = a[0] as u8;
    out[1] = (a[0] >> 8) as u8;
    out[2] = ((a[0] >> 16) | (a[1] << 5)) as u8;
    out[3] = (a[1] >> 3) as u8;
    out[4] = (a[1] >> 11) as u8;
    out[5] = ((a[1] >> 19) | (a[2] << 2)) as u8;
    out[6] = (a[2] >> 6) as u8;
    out[7] = ((a[2] >> 14) | (a[3] << 7)) as u8;
    out[8] = (a[3] >> 1) as u8;
    out[9] = (a[3] >> 9) as u8;
    out[10] = ((a[3] >> 17) | (a[4] << 4)) as u8;
    out[11] = (a[4] >> 4) as u8;
    out[12] = (a[4] >> 12) as u8;
    out[13] = ((a[4] >> 20) | (a[5] << 1)) as u8;
    out[14] = (a[5] >> 7) as u8;
    out[15] = ((a[5] >> 15) | (a[6] << 6)) as u8;
    out[16] = (a[6] >> 2) as u8;
    out[17] = (a[6] >> 10) as u8;
    out[18] = ((a[6] >> 18) | (a[7] << 3)) as u8;
    out[19] = (a[7] >> 5) as u8;
    out[20] = (a[7] >> 13) as u8;
    out[21] = a[8] as u8;
    out[22] = (a[8] >> 8) as u8;
    out[23] = ((a[8] >> 16) | (a[9] << 5)) as u8;
    out[24] = (a[9] >> 3) as u8;
    out[25] = (a[9] >> 11) as u8;
    out[26] = ((a[9] >> 19) | (a[10] << 2)) as u8;
    out[27] = (a[10] >> 6) as u8;
    out[28] = ((a[10] >> 14) | (a[11] << 7)) as u8;
    out[29] = (a[11] >> 1) as u8;
    out[30] = (a[11] >> 9) as u8;
    out[31] = (a[11] >> 17) as u8;
    out
}
