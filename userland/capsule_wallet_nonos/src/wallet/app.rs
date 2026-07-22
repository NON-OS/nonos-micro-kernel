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
use super::state::{hydrate, new_state, State};

pub struct Wallet {
    state: State,
    ready: bool,
    ticks: u32,
    last_active: u32,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet { state: new_state(), ready: false, ticks: 0, last_active: 0 }
    }
}

impl App for Wallet {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        if !self.ready {
            hydrate(&mut self.state);
            self.ready = true;
        }
        // Remember that the user just did something, so the next few ticks skip
        // the blocking network probe and the UI stays instant under the hand.
        self.last_active = self.ticks;
        on_event(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        // Record the width the screens lay out against so pointer handlers can
        // hit-test the same rectangles for width-relative controls.
        self.state.view_w = fb.width;
        paint(&self.state, fb);
    }

    fn on_tick(&mut self) -> bool {
        if !self.ready {
            hydrate(&mut self.state);
            self.ready = true;
            return true;
        }
        self.ticks = self.ticks.wrapping_add(1);
        // Never touch the network while the user is actively interacting: a probe
        // blocks this thread, so defer it until they have paused for a moment.
        // Once idle, refresh one field per pass (never a burst), so a full cycle
        // still lands in roughly the old window without ever stalling a click.
        let idle = self.ticks.wrapping_sub(self.last_active) >= 3;
        if self.state.address_ready && idle && self.ticks % 2 == 0 {
            super::event::probe_tick(&mut self.state);
            return true;
        }
        false
    }
}
