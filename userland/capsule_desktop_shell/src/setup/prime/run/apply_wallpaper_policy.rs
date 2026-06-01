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
use crate::debug;
use crate::wallpaper_client;
pub(super) fn apply_wallpaper_policy(port: u32) -> Result<(), &'static str> {
    if port == 0 {
        return Err("wallpaper service not announced");
    }
    wallpaper_client::queue_policy(port, 3, 0)?;
    debug::marker(b"wallpaper policy deferred");
    Ok(())
}
