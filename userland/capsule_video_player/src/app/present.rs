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

use nonos_app_skeleton::paint::PaintBuffer;

use super::state::VideoApp;
use crate::catalog::entry::file_name;
use crate::player::{column_map, duration_ms, letterbox, permille_of, scale_into};
use crate::ui::chrome::BarState;
use crate::ui::layout::Layout;

impl VideoApp {
    pub(crate) fn title(&self) -> &str {
        if self.path.is_empty() {
            return "Now Playing";
        }
        file_name(&self.path)
    }

    pub(crate) fn bar_state(&self) -> BarState {
        let Some(file) = self.file.as_ref() else {
            return BarState { playing: false, elapsed_ms: 0, total_ms: 0, permille: 0 };
        };
        let total = file.index.len() as u32;
        BarState {
            playing: self.playing,
            elapsed_ms: self.clock.pts_ms(self.next),
            total_ms: duration_ms(total, file.header.micro_sec_per_frame),
            permille: permille_of(self.next, total),
        }
    }

    pub(crate) fn blit_frame(&mut self, fb: &mut PaintBuffer, l: &Layout) -> bool {
        let Some(file) = self.file.as_ref() else {
            return false;
        };
        if self.frame.is_empty() {
            return false;
        }
        let (sw, sh) = (file.video.width, file.video.height);
        if self.geom != (fb.width, fb.height) {
            let (bx, by, bw, bh) = letterbox(sw, sh, l.video.w, l.video.h);
            self.rect = (l.video.x + bx, l.video.y + by, bw, bh);
            self.cols = column_map(sw, bw);
            self.geom = (fb.width, fb.height);
        }
        scale_into(&self.frame, sw, sh, fb.pixels, fb.stride_words, self.rect, &self.cols);
        true
    }
}
