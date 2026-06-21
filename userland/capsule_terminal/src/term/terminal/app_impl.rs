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

use super::types::Terminal;

impl App for Terminal {
    fn manifest(&self) -> AppManifest {
        self.manifest_inner()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        self.on_event_inner(event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.paint_inner(fb)
    }

    // Smoke build only: once services have settled, run a real command
    // through the normal submit path so a headless boot proves command output
    // flows back into the terminal grid. Searches the seeded /docs/demo.txt for
    // a known term so the result is deterministic.
    #[cfg(feature = "nonos-autorun-rg")]
    fn on_tick(&mut self) -> bool {
        if self.autorun_done {
            return false;
        }
        self.autorun_ticks += 1;
        if self.autorun_ticks < 6 {
            return false;
        }
        self.autorun_done = true;
        debug_marker(b"[TERMINAL-AUTORUN] grep demo start\n");
        self.state.line.replace(b"grep nonos < /docs/demo.txt");
        crate::event::on_enter(&mut self.state);
        debug_marker(b"[TERMINAL-AUTORUN] grep demo returned\n");
        true
    }
}

#[cfg(feature = "nonos-autorun-rg")]
fn debug_marker(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}
