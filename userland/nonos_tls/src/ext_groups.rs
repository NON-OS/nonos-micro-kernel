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

use alloc::vec::Vec;

use super::constants::{EXT_SUPPORTED_GROUPS, GROUP_X25519};

pub fn ext_groups(out: &mut Vec<u8>) {
    let mut body = Vec::with_capacity(4);
    super::push::u16(&mut body, 2);
    super::push::u16(&mut body, GROUP_X25519);
    super::push::ext(out, EXT_SUPPORTED_GROUPS, &body);
}
