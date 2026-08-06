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

use super::types::{Blake2b, BLOCK_BYTES};

impl Blake2b {
    pub fn update(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            // Compress only once more input is known to follow: the final
            // block has to be compressed with the last-block flag instead.
            if self.buf_len == BLOCK_BYTES {
                self.counter += BLOCK_BYTES as u128;
                let block = self.buf;
                self.compress(&block, false);
                self.buf_len = 0;
            }
            let take = core::cmp::min(BLOCK_BYTES - self.buf_len, data.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
            self.buf_len += take;
            data = &data[take..];
        }
    }
}
