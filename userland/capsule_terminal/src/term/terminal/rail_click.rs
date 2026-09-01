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

use alloc::vec::Vec;
use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::types::Terminal;
use crate::paint::rail_left_geom::{hit, LeftHit};
use crate::paint::rail_row::inside;

impl Terminal {
    /// The first body-area hit-test in this window. The rail rects come from
    /// the cached solve, so the row a click resolves to is the row the last
    /// frame actually drew there.
    pub(super) fn rail_click(&mut self, event: InputEvent) -> Option<EventOutcome> {
        if !matches!(event.kind, InputKind::ButtonDown) || event.x < 0 || event.y < 0 {
            return None;
        }
        let rail = self.layout?.left_rail;
        let (x, y) = (event.x as u32, event.y as u32);
        if rail.w == 0 || !inside(rail, x, y) {
            return None;
        }
        let projects = self.prefs.project_count as u32;
        Some(match hit(rail, self.tabs.len() as u32, projects, x, y) {
            Some(LeftHit::NewSession) => {
                self.open_tab();
                EventOutcome::Repaint
            }
            Some(LeftHit::AddProject) => self.pin_cwd(),
            Some(LeftHit::Session(i)) => {
                self.active = (i as usize).min(self.tabs.len() - 1);
                EventOutcome::Repaint
            }
            Some(LeftHit::Project(i)) => self.enter_project(i as usize),
            None => EventOutcome::Idle,
        })
    }

    fn pin_cwd(&mut self) -> EventOutcome {
        let path = Vec::from(self.cur_ref().cwd.as_bytes());
        if !self.prefs.push_project(&path) {
            return EventOutcome::Idle;
        }
        self.prefs_dirty = true;
        EventOutcome::Repaint
    }

    pub(super) fn enter_project(&mut self, i: usize) -> EventOutcome {
        let Some(path) = self.prefs.project_slice().get(i).map(|p| Vec::from(p.as_bytes())) else {
            return EventOutcome::Idle;
        };
        self.cur().cwd.set(path);
        EventOutcome::Repaint
    }
}
