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

mod de;
mod es;
mod fr;
mod it;
mod uk;
mod us;

use crate::layout::Layout;

// The letter a layout produces at the physical position of US `base`
// (a..z). Identity except where the layout moves letters.
pub(crate) fn letter(layout: Layout, base: u8) -> u8 {
    match layout {
        Layout::De => de::letter(base),
        Layout::Fr => fr::letter(base),
        _ => base,
    }
}

// The codepoint a layout produces for a non-letter physical key, named by
// its US base character, under the given shift state.
pub(crate) fn symbol(layout: Layout, base: u8, shift: bool) -> u32 {
    match layout {
        Layout::Us => us::symbol(base, shift),
        Layout::Uk => uk::symbol(base, shift),
        Layout::De => de::symbol(base, shift),
        Layout::Fr => fr::symbol(base, shift),
        Layout::Es => es::symbol(base, shift),
        Layout::It => it::symbol(base, shift),
    }
}
