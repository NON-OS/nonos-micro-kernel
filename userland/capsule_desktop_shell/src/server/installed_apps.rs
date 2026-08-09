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

//! Keep the installed capsule-store list in sync with the desktop.

use crate::state::Context;

/// Refresh the installed-app list whenever the installer has a registered
/// service. Most systems never run an installer, so until one appears this
/// costs a bare service lookup and no IPC round trip; once it answers, the
/// list is re-fetched each call and adopted only when it changed.
pub fn load_once(ctx: &mut Context) {
    if !crate::installer_client::available() {
        return;
    }
    let listed = crate::installer_client::list_installed();
    if listed.is_empty() || ctx.installed_apps == listed {
        return;
    }
    ctx.installed_apps = listed;
    ctx.installed_apps_loaded = true;
}
