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

use nonos_desktop::{scene_remove, window_close};
use nonos_runtime::munmap;
use nonos_surface::destroy;

use super::types::DesktopWindow;

impl DesktopWindow {
    pub(crate) fn close(self) {
        let _ = scene_remove(self.peers.compositor, self.rid, 0);
        let _ = destroy(self.handle);
        let _ = destroy(self.handle);
        let _ = munmap(self.backing as *mut u8, self.byte_len as usize);
        let _ = window_close(self.peers.wm, self.rid.wrapping_add(1).max(1), self.window_id);
    }
}
