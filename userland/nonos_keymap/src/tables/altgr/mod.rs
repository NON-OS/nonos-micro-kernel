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
//! The AltGr level, layout by layout.

mod de;
mod es;
mod fr;
mod it;
mod uk;

use crate::layout::Layout;

/// The character AltGr produces on this key, or 0 when the layout puts
/// nothing there and the ordinary level should be used instead.
pub(crate) fn altgr(layout: Layout, base: u8, shift: bool) -> u32 {
    match layout {
        Layout::Us => 0,
        Layout::Uk => uk::altgr(base),
        Layout::De => de::altgr(base),
        Layout::Fr => fr::altgr(base),
        Layout::Es => es::altgr(base),
        Layout::It => it::altgr(base, shift),
    }
}
