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

use super::{AppManifest, EventOutcome};
use crate::input::InputEvent;
use crate::paint::PaintBuffer;

pub trait App {
    fn manifest(&self) -> AppManifest;
    fn on_event(&mut self, event: InputEvent) -> EventOutcome;
    fn paint(&mut self, fb: &mut PaintBuffer);

    fn on_tick(&mut self) -> bool {
        false
    }

    fn tick_interval_ms(&self) -> i64 {
        1000
    }
}
