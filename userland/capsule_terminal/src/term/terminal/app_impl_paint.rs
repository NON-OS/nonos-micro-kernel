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

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_time_millis;

use super::types::Terminal;
impl Terminal {
    pub(super) fn paint_inner(&mut self, fb: &mut PaintBuffer) {
        if self.cur_ref().start_ms == 0 {
            let now = mk_time_millis();
            if now > 0 {
                self.cur().start_ms = now as u64;
            }
        }
        self.width = fb.width;
        let theme = crate::term::theme::profiles::by_index(self.theme);
        crate::paint::paint_tabs(&self.tabs, self.active, fb, theme, self.font_scale, &self.rail);
    }
}
