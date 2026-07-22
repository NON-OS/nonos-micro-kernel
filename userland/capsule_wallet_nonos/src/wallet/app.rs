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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer};

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
        // Only a real click or keypress pauses the background probe, so the UI
        // stays instant under the hand. Mouse movement must NOT count, or moving
        // the cursor would starve the probe and later reads (fee, staking) would
        // never complete.
        if matches!(event.kind, InputKind::ButtonDown | InputKind::KeyDown) {
            self.last_active = self.ticks;
        }
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
        if !self.state.address_ready {
            return false;
        }
        // A probe briefly blocks this thread on network I/O, so only run one
        // while the user has paused (idle). Mouse movement no longer counts as
        // activity, so simply moving the cursor never defers the probe: the
        // reads still land field by field without ever freezing the UI under an
        // active click or keypress.
        let idle = self.ticks.wrapping_sub(self.last_active) >= 3 && self.ticks % 2 == 0;
        if idle {
            super::event::probe_tick(&mut self.state);
            return true;
        }
        false
    }
}
