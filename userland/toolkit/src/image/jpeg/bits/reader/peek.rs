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
    pub fn peek(&mut self, n: u32) -> Result<u32, DecodeError> {
        self.ensure(n)?;
        let shift = self.bit_count - n;
        let mask: u64 = if n == 0 { 0 } else { (1u64 << n) - 1 };
        Ok(((self.bit_buf >> shift) & mask) as u32)
    }
}
