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

//! Keep the list of installable `.nonos` packages in /pkgs in step with the
//! filesystem, so the Launchpad offers whatever is sitting there right now.

use alloc::string::String;
use alloc::vec::Vec;

use crate::state::Context;

/// Rescan /pkgs and adopt the result only when it changed, mirroring the
/// installed-app adoption: a transient empty reply before vfs_pool answers
/// never wipes a list we already have.
pub fn refresh(ctx: &mut Context) {
    let Some(listed) = crate::vfs_client::list(b"/pkgs") else {
        return;
    };
    if listed.is_empty() && !ctx.pkg_files_loaded {
        return;
    }
    let files: Vec<String> = listed
        .into_iter()
        .filter(|e| !e.is_dir && e.name.ends_with(".nonos"))
        .map(|e| e.name)
        .collect();
    if ctx.pkg_files_loaded && ctx.pkg_files == files {
        return;
    }
    ctx.pkg_files = files;
    ctx.pkg_files_loaded = true;
}
