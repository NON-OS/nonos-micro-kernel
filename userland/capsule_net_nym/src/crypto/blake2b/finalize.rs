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

use super::types::Blake2b;

impl Blake2b {
    pub fn finalize(mut self, out: &mut [u8]) {
        self.counter += self.buf_len as u128;
        for byte in self.buf[self.buf_len..].iter_mut() {
            *byte = 0;
        }
        let block = self.buf;
        self.compress(&block, true);
        let n = core::cmp::min(out.len(), self.out_len);
        for i in 0..n {
            out[i] = (self.h[i / 8] >> (8 * (i % 8))) as u8;
        }
    }
}
