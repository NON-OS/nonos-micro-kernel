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

use super::selector::Selector;

pub fn specificity(sel: &Selector) -> u32 {
    let (mut ids, mut classes, mut tags) = (0u32, 0u32, 0u32);
    for s in core::iter::once(&sel.key).chain(sel.ancestors.iter().map(|a| &a.simple)) {
        ids += s.id.is_some() as u32;
        classes += s.classes.len() as u32 + s.attrs.len() as u32;
        tags += s.tag.is_some() as u32;
    }
    // CSS 2.1 6.4.3 compares specificity as the tuple (ids, classes+attrs,
    // tags): a higher level always outranks any count at a lower one, so one id
    // beats any number of classes. Pack into a single comparable u32 with ten
    // bits per level so a lower level can never carry into a higher one; clamp
    // each so a pathological selector cannot wrap.
    (ids.min(0x3FF) << 20) | (classes.min(0x3FF) << 10) | tags.min(0x3FF)
}
