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
use crate::image::types::DecodeError;

use super::BitReader;

impl BitReader<'_> {
    pub(in crate::image::jpeg::bits::reader) fn fill_byte(&mut self) -> Result<(), DecodeError> {
        if self.marker_hit.is_some() {
            self.bit_buf <<= 8;
            self.bit_count += 8;
            return Ok(());
        }
        if self.pos >= self.data.len() {
            return Err(DecodeError::Truncated);
        }
        let mut b = self.data[self.pos];
        self.pos += 1;
        if b == 0xFF {
            if self.pos >= self.data.len() {
                return Err(DecodeError::Truncated);
            }
            let next = self.data[self.pos];
            self.pos += 1;
            if next == 0x00 {
                b = 0xFF;
            } else {
                self.marker_hit = Some(next);
                self.bit_buf <<= 8;
                self.bit_count += 8;
                return Ok(());
            }
        }
        self.bit_buf = (self.bit_buf << 8) | (b as u64);
        self.bit_count += 8;
        Ok(())
    }
}
