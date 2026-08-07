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

// Idle ticks between account refreshes. One batched fetch fills every field,
// so the wallet does not need to poll often; a refresh every dozen seconds
// keeps balances current while leaving the UI free almost all the time.
const REFRESH_TICKS: u32 = 12;

pub struct Wallet {
    state: State,
    ready: bool,
    ticks: u32,
    last_active: u32,
    last_probe: u32,
    probed_once: bool,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            state: new_state(),
            ready: false,
            ticks: 0,
            last_active: 0,
            last_probe: 0,
            probed_once: false,
        }
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
        self.state.view_h = fb.height;
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
        // Refresh only when the user has paused AND either it has never fetched
        // yet or a full refresh interval has elapsed. One batched fetch fills
        // every field, so between refreshes the UI does no network I/O at all
        // and stays fully responsive. Any click or keypress pauses it, so a
        // fetch never lands under the hand.
        let idle = self.ticks.wrapping_sub(self.last_active) >= 2;
        let due = !self.probed_once || self.ticks.wrapping_sub(self.last_probe) >= REFRESH_TICKS;
        if idle && due {
            self.last_probe = self.ticks;
            self.probed_once = true;
            // Repaint only when the probe changed something on screen, so a
            // steady balance does not recomposite the window.
            return matches!(super::event::probe_tick(&mut self.state), EventOutcome::Repaint);
        }
        false
    }
}
