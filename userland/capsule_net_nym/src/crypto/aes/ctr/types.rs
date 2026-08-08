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

use super::super::types::BLOCK_BYTES;
use super::super::Aes128;

pub struct Ctr64Be {
    pub(super) cipher: Aes128,
    pub(super) counter: [u8; BLOCK_BYTES],
}

impl Ctr64Be {
    pub fn new(key: &[u8; super::super::KEY_BYTES], iv: &[u8; BLOCK_BYTES]) -> Self {
        Self { cipher: Aes128::new(key), counter: *iv }
    }
}
