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


use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::types::Terminal;
use crate::paint::rail_row::inside;
use crate::paint::rail_scroll::{clamp, RailFit};

/// Pixels the column moves per wheel notch, a shade over one list row so a
/// single notch always changes what is on screen.
const WHEEL_PX: u32 = 48;

impl Terminal {
    /// The column's shape as both the painter and the hit-test see it.
    pub(super) fn rail_fit(&self) -> RailFit {
        RailFit {
            sessions: self.tabs.len() as u32,
            projects: self.prefs.project_count as u32,
            telemetry: self.prefs.rails & 1 == 0,
            procs: self.rail.sample.n as u32,
        }
    }

    /// A notch over the rail moves the column; anywhere else the event falls
    /// through to the scrollback, which is where it went before the rail
    /// learned to scroll at all.
    pub(super) fn rail_wheel(&mut self, event: InputEvent) -> Option<EventOutcome> {
        if event.kind != InputKind::Wheel || event.x < 0 || event.y < 0 {
            return None;
        }
        let rail = self.layout?.left_rail;
        let (x, y) = (event.x as u32, event.y as u32);
        if rail.w == 0 || !inside(rail, x, y) {
            return None;
        }
        let step = (event.delta_y.unsigned_abs()).min(10) * WHEEL_PX;
        let want = if event.delta_y > 0 {
            self.rail_scroll.saturating_sub(step)
        } else {
            self.rail_scroll.saturating_add(step)
        };
        let next = clamp(want, self.rail_fit(), rail.h);
        let moved = next != self.rail_scroll;
        self.rail_scroll = next;
        Some(if moved { EventOutcome::Repaint } else { EventOutcome::Idle })
    }
}
