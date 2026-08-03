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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::frame;
use super::manifest::manifest;
use super::paint::paint;

pub struct EguiProof {
    ctx: egui::Context,
    clicks: u32,
    frames: u32,
}

impl EguiProof {
    pub fn new() -> Self {
        EguiProof {
            ctx: egui::Context::default(),
            clicks: 0,
            frames: 0,
        }
    }
}

impl App for EguiProof {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, _event: InputEvent) -> EventOutcome {
        self.clicks = self.clicks.wrapping_add(1);
        EventOutcome::Repaint
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.frames = self.frames.wrapping_add(1);
        let stats = frame::run(&self.ctx, self.clicks);
        paint(fb, self.frames, stats);
    }
}
