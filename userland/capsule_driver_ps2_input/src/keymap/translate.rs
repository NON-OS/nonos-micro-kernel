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
use super::set1::keycode_for as base_keycode_for;
use super::set1_e0::keycode_for as e0_keycode_for;
use crate::ring::{FLAG_BREAK, FLAG_E0_PREFIX};
pub struct Translated {
    pub keycode: u32,
    pub is_release: bool,
}
pub fn translate(scancode: u8, flags: u8) -> Option<Translated> {
    let is_release = (flags & FLAG_BREAK) != 0;
    let key = scancode & 0x7F;
    let keycode =
        if (flags & FLAG_E0_PREFIX) != 0 { e0_keycode_for(key)? } else { base_keycode_for(key)? };
    Some(Translated { keycode, is_release })
}
