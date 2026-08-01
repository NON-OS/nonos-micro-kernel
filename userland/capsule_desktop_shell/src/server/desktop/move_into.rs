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

//! Move a desktop item into a folder by renaming it under that folder's path,
//! then re-sync. Both indices are into the root listing.

use alloc::format;
use nonos_libc::mk_time_millis;

use crate::state::{Context, NotifyLevel};

pub fn move_into(ctx: &mut Context, src: usize, folder: usize) {
    if src == folder {
        return;
    }
    let src_name = match ctx.desktop_items.get(src) {
        Some(item) => item.name.clone(),
        None => return,
    };
    let folder_item = match ctx.desktop_items.get(folder) {
        Some(item) => item,
        None => return,
    };
    if !folder_item.is_dir {
        return;
    }
    let old = format!("/{src_name}");
    let new = format!("/{}/{}", folder_item.name, src_name);
    if crate::vfs_client::rename(old.as_bytes(), new.as_bytes()) {
        let _ = super::refresh::refresh(ctx);
    } else {
        // The icon just sprang back to where it started, unexplained.
        let now = mk_time_millis();
        ctx.toasts.push(b"could not move", NotifyLevel::Error, now);
    }
}
