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

pub fn u64_decimal(mut value: u64, dst: &mut [u8; 20]) -> &[u8] {
    if value == 0 {
        dst[0] = b'0';
        return &dst[..1];
    }
    let mut tmp = [0u8; 20];
    let mut idx = 0;
    while value > 0 {
        tmp[idx] = b'0' + (value % 10) as u8;
        value /= 10;
        idx += 1;
    }
    let mut out_len = 0;
    while idx > 0 {
        idx -= 1;
        dst[out_len] = tmp[idx];
        out_len += 1;
    }
    &dst[..out_len]
}

