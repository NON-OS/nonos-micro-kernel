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

//! Pull the desktop back in sync with the filesystem.

use crate::state::Context;

/// Reload the root listing. A transient empty reply, which happens while
/// vfs_pool is still coming up or briefly busy after a write, never clears a
/// good desktop. We only adopt a new listing when it has entries and differs
/// from what is shown. Returns whether the desktop changed and wants a repaint.
pub fn refresh(ctx: &mut Context) -> bool {
    let items = crate::vfs_client::list(b"/");
    if items.is_empty() || super::same::same(&ctx.desktop_items, &items) {
        return false;
    }
    ctx.desktop_items = items;
    true
}
