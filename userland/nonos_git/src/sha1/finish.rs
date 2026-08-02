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

//! Padding and the final digest.

use super::state::Sha1;

impl Sha1 {
    /// Pad with 0x80, then zeros, then the 64-bit bit length.
    pub fn finish(mut self) -> [u8; 20] {
        let bit_len = self.len.wrapping_mul(8);
        let pad = [0u8; 72];
        let rem = ((self.len % 64) as usize + 1) + 8;
        let pad_len = if rem <= 64 { 64 - rem + 1 } else { 128 - rem + 1 };
        let mut first = [0u8; 72];
        first[0] = 0x80;
        first[1..pad_len].copy_from_slice(&pad[1..pad_len]);
        self.update(&first[..pad_len]);
        self.update(&bit_len.to_be_bytes());

        let mut out = [0u8; 20];
        for (i, word) in self.state.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}
