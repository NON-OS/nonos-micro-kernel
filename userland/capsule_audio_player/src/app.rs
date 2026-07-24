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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer, WindowKind};

const WINDOW_ID: u32 = 0x5245_534E;
const BG: u32 = 0xFF10161C;

pub struct PlayerApp;

impl PlayerApp {
    pub fn new() -> Self {
        PlayerApp
    }
}

impl App for PlayerApp {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"Resonare",
            window_id: WINDOW_ID,
            kind: WindowKind::Normal,
            initial_x: 360,
            initial_y: 240,
            width: 480,
            height: 320,
            input_kind_mask: 0,
        }
    }
    fn on_event(&mut self, _event: InputEvent) -> EventOutcome {
        EventOutcome::Idle
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        fb.clear(BG);
    }
}
