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

use super::types::Terminal;
use crate::jobs;
use crate::term::prefs::store;

// `tick_interval_ms` is 30, so a whole second of ticks separates two writes and
// a burst of zoom keystrokes collapses into one VFS round trip.
const SAVE_TICKS: u32 = 34;

impl Terminal {
    pub(super) fn on_tick_inner(&mut self) -> bool {
        self.flush_prefs();
        jobs::pump(self.cur())
    }

    fn flush_prefs(&mut self) {
        if !self.prefs_dirty {
            self.prefs_ticks = 0;
            return;
        }
        self.prefs_ticks += 1;
        if self.prefs_ticks < SAVE_TICKS {
            return;
        }
        self.prefs_ticks = 0;
        self.prefs_dirty = false;
        self.prefs.theme = self.theme;
        self.prefs.font_scale = self.font_scale as u8;
        store::save(&self.prefs);
    }
}
