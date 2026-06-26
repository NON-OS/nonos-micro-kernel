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
use super::state::State;

pub struct Browser {
    state: State,
}

impl Browser {
    pub fn new() -> Self {
        Browser { state: State::new() }
    }
}

impl App for Browser {
    fn manifest(&self) -> AppManifest {
        manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.state, event)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(&self.state, fb);
    }
    fn on_tick(&mut self) -> bool {
        if self.state.fetch_job.is_some() {
            self.state.pending_nav = None;
            return crate::browser::fetch::poll(&mut self.state);
        }
        if let Some(target) = self.state.pending_nav.take() {
            crate::browser::fetch::load(&mut self.state, &target);
            return true;
        }
        false
    }
    fn tick_interval_ms(&self) -> i64 {
        50
    }
}
