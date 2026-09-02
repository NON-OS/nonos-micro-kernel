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

use super::types::Terminal;

impl App for Terminal {
    fn manifest(&self) -> AppManifest {
        self.manifest_inner()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        self.on_event_inner(event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.paint_inner(fb)
    }

    fn on_tick(&mut self) -> bool {
        self.on_tick_inner()
    }

    fn tick_interval_ms(&self) -> i64 {
        30
    }

    fn titlebar_accessory_w(&self) -> u32 {
        self.accessory_w()
    }

    fn paint_accessory(&mut self, fb: &mut PaintBuffer) {
        self.paint_accessory_inner(fb)
    }

    fn on_accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        self.accessory_event(event)
    }
}
