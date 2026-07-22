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
}

impl Wallet {
    pub fn new() -> Self {
        Wallet { state: new_state(), ready: false, ticks: 0 }
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
        on_event(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(&self.state, fb);
    }

    fn on_tick(&mut self) -> bool {
        if !self.ready {
            hydrate(&mut self.state);
            self.ready = true;
            return true;
        }
        self.ticks = self.ticks.wrapping_add(1);
        // Refresh one field per pass, never a burst. Each pass blocks on at most
        // one RPC round-trip, so clicks are always serviced between them; a full
        // eight-field cycle lands in roughly the old ~15s window.
        if self.state.address_ready && self.ticks % 2 == 0 {
            super::event::probe_tick(&mut self.state);
            return true;
        }
        false
    }
}
