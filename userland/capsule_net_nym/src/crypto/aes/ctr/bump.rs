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

use super::types::Ctr64Be;

impl Ctr64Be {
    /// Increment the low 64 bits big-endian, wrapping within them. The high
    /// half is the nonce prefix and never moves; a 128-bit counter would agree
    /// for the first 2^64 blocks and still be the wrong cipher.
    pub(super) fn bump(&mut self) {
        let low = u64::from_be_bytes([
            self.counter[8],
            self.counter[9],
            self.counter[10],
            self.counter[11],
            self.counter[12],
            self.counter[13],
            self.counter[14],
            self.counter[15],
        ]);
        self.counter[8..].copy_from_slice(&low.wrapping_add(1).to_be_bytes());
    }
}
