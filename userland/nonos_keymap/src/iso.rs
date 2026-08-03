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
//! The extra key an ISO keyboard has that an ANSI one does not.

use crate::layout::Layout;

/// The key between the left shift and Z on a 102 or 105 key board. It has no
/// US base character, so it is named by a code of its own and resolved here.
/// The value sits clear of the 0xE000 page the drivers post navigation keys
/// in, since this one is only ever a driver-internal name for a position.
pub const KEY_ISO: u32 = 0xE100;

/// What that key produces, or 0 when the layout has no such key and the
/// press should be dropped rather than turned into a character.
pub fn iso(layout: Layout, shift: bool, altgr: bool) -> u32 {
    match layout {
        // An ANSI board has nothing in this position.
        Layout::Us => 0,
        Layout::Uk => {
            if shift {
                b'|' as u32
            } else {
                b'\\' as u32
            }
        }
        Layout::De | Layout::Es if altgr => b'|' as u32,
        _ => {
            if shift {
                b'>' as u32
            } else {
                b'<' as u32
            }
        }
    }
}
