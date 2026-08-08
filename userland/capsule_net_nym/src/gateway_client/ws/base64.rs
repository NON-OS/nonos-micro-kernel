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

const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(input: &[u8], out: &mut [u8]) -> Option<usize> {
    let need = ((input.len() + 2) / 3) * 4;
    if out.len() < need {
        return None;
    }
    let mut i = 0;
    let mut j = 0;
    while i < input.len() {
        let b0 = input[i];
        let b1 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] } else { 0 };
        out[j] = TABLE[(b0 >> 2) as usize];
        out[j + 1] = TABLE[(((b0 & 3) << 4) | (b1 >> 4)) as usize];
        out[j + 2] =
            if i + 1 < input.len() { TABLE[(((b1 & 15) << 2) | (b2 >> 6)) as usize] } else { b'=' };
        out[j + 3] = if i + 2 < input.len() { TABLE[(b2 & 63) as usize] } else { b'=' };
        i += 3;
        j += 4;
    }
    Some(need)
}
