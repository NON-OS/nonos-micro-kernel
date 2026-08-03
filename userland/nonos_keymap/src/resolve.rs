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

use crate::layout::Layout;
use crate::tables;

/// Resolve a physical key, named by the US base character its position
/// carries in the scancode tables, into the final codepoint for `layout`.
///
/// Letter keys go through the layout's position remap first (AZERTY moves
/// a/q, z/w and m; QWERTZ swaps y/z), then case: shift XOR caps-lock,
/// matching how caps behaves on real keyboards (caps only affects
/// letters). Every other key resolves through the layout's symbol table
/// with the shift state alone.
///
/// AltGr is checked first and only wins where the layout puts something
/// on the third level, so holding it over a key with nothing there gives
/// the ordinary character rather than nothing at all. Codepoints outside
/// the printable ASCII base range (navigation, function and modifier
/// keys) pass through untouched, as do values that are not a US base
/// character.
pub fn resolve(base: u32, shift: bool, caps: bool, altgr: bool, layout: Layout) -> u32 {
    if base == crate::iso::KEY_ISO {
        return crate::iso::iso(layout, shift, altgr);
    }
    if !(0x20..=0x7E).contains(&base) {
        return base;
    }
    let b = base as u8;
    if altgr {
        let c = tables::altgr(layout, b, shift);
        if c != 0 {
            return c;
        }
    }
    // AZERTY's M key sits where US ';' is; resolve it before the generic
    // letter path so it gets letter casing, and let the US 'm' position
    // fall through to the symbol table (it produces ',' / '?' there).
    if layout == Layout::Fr {
        if b == b';' {
            return cased(b'm', shift, caps);
        }
        if b == b'm' {
            return tables::symbol(layout, b, shift);
        }
    }
    if b.is_ascii_lowercase() {
        return cased(tables::letter(layout, b), shift, caps);
    }
    tables::symbol(layout, b, shift)
}

fn cased(letter: u8, shift: bool, caps: bool) -> u32 {
    if shift ^ caps {
        letter.to_ascii_uppercase() as u32
    } else {
        letter as u32
    }
}
