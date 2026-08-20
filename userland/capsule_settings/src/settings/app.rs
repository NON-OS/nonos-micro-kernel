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

use super::event::{on_accessory, on_event};
use super::ipc::{hydrate, lookup_policy_port};
use super::manifest::manifest;
use super::paint::paint;
use super::section::Section;
use super::state::refresh_wifi::refresh_wifi_status;
use super::state::{state_new, State};
use super::ui::search_field;

pub struct Settings {
    state: State,
    hydrated: bool,
}

impl Settings {
    pub fn new() -> Self {
        let mut state = state_new();
        if let Ok(port) = lookup_policy_port() {
            state.policy_port = port;
            state.policy_ready = true;
        }
        Settings { state, hydrated: false }
    }
}

impl Settings {
    fn ensure_ready(&mut self) {
        if !self.state.policy_ready {
            if let Ok(port) = lookup_policy_port() {
                self.state.policy_port = port;
                self.state.policy_ready = true;
            }
        }
        if self.state.policy_ready && !self.hydrated {
            hydrate(&mut self.state);
            self.hydrated = true;
        }
    }
}

impl App for Settings {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        self.ensure_ready();
        on_event(&mut self.state, event)
    }

    fn titlebar_accessory_w(&self) -> u32 {
        search_field::WIDTH
    }

    fn paint_accessory(&mut self, fb: &mut PaintBuffer) {
        search_field::paint(fb, &self.state);
    }

    fn on_accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        on_accessory(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.ensure_ready();
        // The window resizes, so record what is actually being painted into.
        // Hit tests and row counts read this back.
        self.state.win_w = fb.width;
        self.state.win_h = fb.height;
        paint(&self.state, fb);
    }

    // While the Wi-Fi panel is open, re-poll net_core every tick so a lease that
    // binds a few seconds after connecting shows its address without the user
    // having to trigger another scan.
    fn on_tick(&mut self) -> bool {
        if self.state.section == Section::Wifi {
            refresh_wifi_status(&mut self.state);
            return true;
        }
        false
    }
}
