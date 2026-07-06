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

use crate::clock::manifest::manifest;
use crate::clock::paint::paint;
use crate::clock::state::State;

pub struct Clock {
    state: State,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            state: State::new(),
        }
    }
}

impl App for Clock {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, _event: InputEvent) -> EventOutcome {
        EventOutcome::Idle
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(&self.state, fb);
    }

    fn on_tick(&mut self) -> bool {
        self.state.refresh();
        true
    }
}
