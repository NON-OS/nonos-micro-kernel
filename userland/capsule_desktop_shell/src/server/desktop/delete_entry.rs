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

//! Delete the desktop item at `index` from the filesystem, then re-sync.

use alloc::string::String;
use nonos_libc::mk_time_millis;

use crate::state::{Context, NotifyLevel};

pub fn delete_entry(ctx: &mut Context, index: usize) {
    let (name, is_dir) = match ctx.desktop_items.get(index) {
        Some(item) => (item.name.clone(), item.is_dir),
        None => return,
    };
    let mut path = String::from("/");
    path.push_str(&name);
    if crate::vfs_client::remove(path.as_bytes(), is_dir) {
        let _ = super::refresh::refresh(ctx);
    } else {
        // A refused delete left the icon in place with no explanation.
        let now = mk_time_millis();
        ctx.toasts.push(b"could not delete", NotifyLevel::Error, now);
    }
}
