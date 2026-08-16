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
use crate::player::{column_map, duration_ms, letterbox, permille_of, scale_into};
use crate::ui::chrome::{paint_bar, BarState};
use crate::ui::layout::layout;

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

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        crate::event::router::on_event(self, event)
    }

    fn on_tick(&mut self) -> bool {
        self.advance()
    }

    fn tick_interval_ms(&self) -> i64 {
        if self.playing || self.force_decode {
            4
        } else {
            250
        }
    }

    fn busy(&self) -> bool {
        self.playing
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.dims = (fb.width, fb.height);
        for px in fb.pixels.iter_mut() {
            *px = BACKDROP;
        }
        let Some(file) = self.file.as_ref() else {
            return;
        };
        let (sw, sh) = (file.video.width, file.video.height);
        let l = layout(fb.width, fb.height);
        if self.geom != (fb.width, fb.height) {
            let (bx, by, bw, bh) = letterbox(sw, sh, l.video.w, l.video.h);
            self.rect = (l.video.x + bx, l.video.y + by, bw, bh);
            self.cols = column_map(sw, bw);
            self.geom = (fb.width, fb.height);
        }
        scale_into(&self.frame, sw, sh, fb.pixels, fb.stride_words, self.rect, &self.cols);
        let total = file.index.len() as u32;
        let upf = file.header.micro_sec_per_frame;
        let st = BarState {
            playing: self.playing,
            elapsed_ms: self.clock.pts_ms(self.next),
            total_ms: duration_ms(total, upf),
            permille: permille_of(self.next, total),
        };
        paint_bar(fb, &l, &st);
    }
}
