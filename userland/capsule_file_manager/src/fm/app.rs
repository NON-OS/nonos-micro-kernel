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
use super::paint::paint;
use super::refresh::refresh;
use super::state::State;

pub struct FileManager {
    state: State,
}

impl FileManager {
    pub fn new() -> Self {
        let mut state = State::new();
        refresh(&mut state);
        FileManager { state }
    }
}

impl App for FileManager {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        if self.state.owner_pid == 0 || self.state.status == b"vfs unavailable" {
            refresh(&mut self.state);
        }
        on_event(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        if self.state.owner_pid == 0 || self.state.status == b"vfs unavailable" {
            refresh(&mut self.state);
        }
        super::layout::measure(&mut self.state, fb.height);
        paint(&self.state, fb);
    }
}
