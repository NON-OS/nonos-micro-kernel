// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 NONOS
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Activity-bar routing shared by every screen.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::activity_bar::activity_hit;
use super::app::Editor;
use super::layout::ACTIVITY_W;

impl Editor {
    pub(super) fn activity_event(&mut self, event: &InputEvent) -> Option<EventOutcome> {
        if event.kind != InputKind::ButtonDown {
            return None;
        }
        if event.x < 0 || event.x >= ACTIVITY_W as i32 {
            return None;
        }
        Some(self.activity_press(activity_hit(event.y)))
    }
}
