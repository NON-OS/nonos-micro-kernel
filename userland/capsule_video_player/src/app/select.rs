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

use alloc::string::ToString;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::state::VideoApp;
use crate::catalog::scan::scan;
use crate::ui::screen::Route;

impl VideoApp {
    pub(crate) fn refresh_library(&mut self) {
        if self.browse.scanned {
            return;
        }
        self.browse.scanned = true;
        self.browse.items = scan(mk_getpid());
        self.browse.reindex();
    }

    pub(crate) fn open_index(&mut self, slot: usize) -> bool {
        let Some(item) = self.browse.get(slot) else {
            return false;
        };
        if !item.decodable() {
            self.status = Some("format not supported yet");
            self.browse.sel = slot;
            return true;
        }
        self.path = item.path.to_string();
        self.browse.sel = slot;
        self.reset_playback();
        self.nav.go(Route::Player);
        self.playing = true;
        true
    }

    fn reset_playback(&mut self) {
        self.opened = false;
        self.source = None;
        self.file = None;
        self.frame = Vec::new();
        self.cols = Vec::new();
        self.geom = (0, 0);
        self.next = 0;
        self.status = None;
    }
}
