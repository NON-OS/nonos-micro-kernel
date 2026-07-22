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
use super::state::{hydrate, needs_live_data, new_state, State};

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
        // A probe briefly blocks this thread on network I/O, so it runs only
        // when three things hold: the user has paused (idle), no text field is
        // open, and the current screen actually shows live data. That keeps the
        // Receive screen, where the account is generated, imported, backed up
        // and recovered, completely free of blocking reads, so those flows
        // never stall, while balances and fees still refresh on the screens
        // that display them.
        if !needs_live_data(self.state.view)
            || self.state.import_active
            || self.state.recover_active
            || self.state.backup_active
        {
            return false;
        }
        // Once the user has paused, refresh one field every idle tick so the
        // whole cycle (balance, nonce, fee, staking) fills in a few seconds
        // rather than dribbling in. Any real click or keypress pauses it again
        // immediately, so this never blocks the hand.
        let idle = self.ticks.wrapping_sub(self.last_active) >= 2;
        if idle {
            // Repaint only when the probe changed something on screen, so a
            // steady balance does not recomposite the window every cycle.
            return matches!(super::event::probe_tick(&mut self.state), EventOutcome::Repaint);
        }
        false
    }
}
