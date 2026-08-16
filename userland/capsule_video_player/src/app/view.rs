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

use nonos_app_skeleton::app::{App, AppManifest, EventOutcome, WindowKind};
use nonos_app_skeleton::input::InputEvent;
use nonos_app_skeleton::paint::PaintBuffer;

use super::state::{VideoApp, WINDOW_ID};
use crate::player::{column_map, letterbox, scale_into};

const BACKDROP: u32 = 0xff00_0000;
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

impl App for VideoApp {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"Video",
            window_id: WINDOW_ID,
            kind: WindowKind::Normal,
            initial_x: 160,
            initial_y: 120,
            width: 960,
            height: 600,
            input_kind_mask: INPUT_KEY_DOWN_BIT,
        }
    }

    fn on_event(&mut self, _event: InputEvent) -> EventOutcome {
        EventOutcome::Idle
    }

    fn on_tick(&mut self) -> bool {
        self.advance()
    }

    fn tick_interval_ms(&self) -> i64 {
        if self.playing {
            4
        } else {
            250
        }
    }

    fn busy(&self) -> bool {
        self.playing
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        for px in fb.pixels.iter_mut() {
            *px = BACKDROP;
        }
        let Some(file) = self.file.as_ref() else {
            return;
        };
        let (sw, sh) = (file.video.width, file.video.height);
        if self.geom != (fb.width, fb.height) {
            self.rect = letterbox(sw, sh, fb.width, fb.height);
            self.cols = column_map(sw, self.rect.2);
            self.geom = (fb.width, fb.height);
        }
        scale_into(&self.frame, sw, sh, fb.pixels, fb.stride_words, self.rect, &self.cols);
    }
}
