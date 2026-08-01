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

//! Create a file or folder from the desktop right-click menu.

use alloc::string::String;
use nonos_libc::mk_time_millis;

use crate::state::{Context, NotifyLevel};

/// Create a new file or folder at the root under a non-colliding name and pull
/// the desktop back in sync. Repainting is left to the caller so it can first
/// tidy up its own state, such as closing the menu that triggered this.
pub fn create_entry(ctx: &mut Context, is_file: bool) {
    let base = if is_file { "New File" } else { "New Folder" };
    let name = super::unique_name::unique_name(ctx, base);
    let mut path = String::from("/");
    path.push_str(&name);
    let created = if is_file {
        crate::vfs_client::create_file(path.as_bytes())
    } else {
        crate::vfs_client::mkdir(path.as_bytes())
    };
    if created {
        let _ = super::refresh::refresh(ctx);
    } else {
        // A full or read-only volume produced nothing and said nothing.
        let now = mk_time_millis();
        ctx.toasts.push(b"could not create", NotifyLevel::Error, now);
    }
}
