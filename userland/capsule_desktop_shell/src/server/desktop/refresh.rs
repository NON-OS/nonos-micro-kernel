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

/// Reload the root listing. A failed call, which happens while vfs_pool is
/// still coming up or is briefly busy after a write, never clears a good
/// desktop. An answer that is genuinely empty is adopted: keying on
/// non-emptiness meant deleting your last item left its icon painted until
/// something else was created. Returns whether the desktop wants a repaint.
pub fn refresh(ctx: &mut Context) -> bool {
    let Some(items) = crate::vfs_client::list(b"/") else {
        return false;
    };
    if super::same::same(&ctx.desktop_items, &items) {
        return false;
    }
    ctx.desktop_items = items;
    true
}
