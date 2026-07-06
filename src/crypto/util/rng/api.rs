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

use super::global::{fill_random_bytes, random_u64_secure};

#[inline]
pub fn fill_bytes(buffer: &mut [u8]) {
    fill_random_bytes(buffer);
}

#[inline]
pub fn secure_random_u64() -> u64 {
    match random_u64_secure() {
        Ok(v) => v,
        Err(_) => loop {
            core::hint::spin_loop();
        },
    }
}
