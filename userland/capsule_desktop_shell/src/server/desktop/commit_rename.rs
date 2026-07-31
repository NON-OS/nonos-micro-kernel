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

//! Finish an inline rename: rename the entry on the filesystem and re-sync.

use alloc::format;
use alloc::string::ToString;
use nonos_libc::mk_time_millis;

use crate::state::{Context, NotifyLevel};

pub fn commit_rename(ctx: &mut Context) {
    super::release_keys::release_keys(ctx);
    let Some(index) = ctx.rename.take() else {
        return;
    };
    let new_name = ctx.rename_buf.trim().to_string();
    ctx.rename_buf.clear();

    let old_name = match ctx.desktop_items.get(index) {
        Some(item) => item.name.clone(),
        None => return,
    };
    if new_name.is_empty() || new_name == old_name || new_name.contains('/') {
        return;
    }
    let old = format!("/{old_name}");
    let new = format!("/{new_name}");
    if crate::vfs_client::rename(old.as_bytes(), new.as_bytes()) {
        let _ = super::refresh::refresh(ctx);
    } else {
        // The name silently snapped back to the old one before.
        let now = mk_time_millis();
        ctx.toasts.push(b"could not rename", NotifyLevel::Error, now);
    }
}
