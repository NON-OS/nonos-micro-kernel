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
use crate::ui::icon;
use crate::ui::layout::layout;
use crate::ui::player::header::paint_header;
use crate::ui::player::paint::paint_transport;
use crate::ui::screen::Route;
use crate::ui::theme;
use crate::ui::view::render::paint_route;
use crate::ui::widget::empty::paint_empty;

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
        if self.route() != Route::Player {
            paint_route(fb, self);
            return;
        }
        for px in fb.pixels.iter_mut() {
            *px = theme::BACKDROP;
        }
        let l = layout(fb.width, fb.height);
        let (w, title) = (fb.width, self.title());
        paint_header(fb, w, title);
        let st = self.bar_state();
        if !self.blit_frame(fb, &l) {
            let note = self.status.unwrap_or("Open a video from your library to start playback");
            paint_empty(fb, l.video, icon::nav::video, "Nothing playing", note);
        }
        paint_transport(fb, &l, &st, self.muted, self.volume);
    }
}
