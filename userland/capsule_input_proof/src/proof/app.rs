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

use super::emit::Line;
use super::manifest::manifest;
use super::markers::on_input;
use super::state::Latches;

const BACKGROUND_ARGB: u32 = 0xFF10_1820;

pub struct InputProof {
    latches: Latches,
}

impl InputProof {
    pub fn new() -> Self {
        InputProof { latches: Latches::new() }
    }
}

impl App for InputProof {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_input(&mut self.latches, &event);
        EventOutcome::Idle
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        if !self.latches.composited {
            self.latches.composited = true;
            Line::new().push(b"surface composited").emit();
        }
        fb.clear(BACKGROUND_ARGB);
    }
}
