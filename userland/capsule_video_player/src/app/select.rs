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

use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::state::VideoApp;
use crate::catalog::scan::scan;
use crate::ui::screen::Screen;

impl VideoApp {
    pub(crate) fn refresh_library(&mut self) {
        if self.scanned {
            return;
        }
        self.scanned = true;
        self.items = scan(mk_getpid());
        if self.sel >= self.items.len() {
            self.sel = 0;
        }
    }

    pub(crate) fn open_index(&mut self, index: usize) -> bool {
        let Some(path) = self.items.get(index) else {
            return false;
        };
        self.path = path.clone();
        self.sel = index;
        self.screen = Screen::Player;
        self.opened = false;
        self.source = None;
        self.file = None;
        self.frame = Vec::new();
        self.cols = Vec::new();
        self.geom = (0, 0);
        self.next = 0;
        self.playing = true;
        self.status = None;
        true
    }
}
