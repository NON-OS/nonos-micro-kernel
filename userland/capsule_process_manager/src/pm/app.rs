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

use super::event::on_event;
use super::manifest::manifest;
use super::state::State;
use super::ui::paint::paint;

pub struct ProcessManager {
    state: State,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager { state: State::new() }
    }
}

impl App for ProcessManager {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(&mut self.state, fb);
    }

    fn on_tick(&mut self) -> bool {
        // The default tick is one second, a natural refresh rate for a monitor.
        // Re-read the live table and always repaint: uptime advances every tick
        // for every process, so the view is genuinely live rather than a frame
        // that only redraws when a number happens to change. One repaint a second
        // of cached glyphs is cheap.
        self.state.refresh();
        true
    }
}
