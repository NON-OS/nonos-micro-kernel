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

//! Choose a fresh name for a new desktop entry.

use alloc::format;
use alloc::string::String;

use crate::state::Context;

/// Return a name that does not collide with an existing entry, following the
/// familiar "New Folder", "New Folder 2", ... progression.
pub(super) fn unique_name(ctx: &Context, base: &str) -> String {
    let taken = |name: &str| ctx.desktop_items.iter().any(|e| e.name == name);
    let mut name = String::from(base);
    let mut n = 2u32;
    while taken(&name) {
        name = format!("{base} {n}");
        n += 1;
    }
    name
}
