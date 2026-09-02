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

use crate::roots::{chunk0, chunk1, chunk2, chunk3, extra};

pub fn is_trusted_spki_hash(h: &[u8; 32]) -> bool {
    chunk0::ROOTS_0.iter().any(|r| r == h)
        || chunk1::ROOTS_1.iter().any(|r| r == h)
        || chunk2::ROOTS_2.iter().any(|r| r == h)
        || chunk3::ROOTS_3.iter().any(|r| r == h)
        || extra::EXTRA_ROOTS.iter().any(|r| r == h)
}
