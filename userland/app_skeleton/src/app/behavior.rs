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

    /// Whether the app has pending asynchronous work, such as an in-flight
    /// network request or a load in progress, and needs to keep ticking
    /// promptly. When true the runner yields cooperatively between frames
    /// instead of sleeping to the next vblank, so the work advances even where
    /// the scheduler's periodic wake is unreliable and a frame would otherwise
    /// stall until the next input event. Defaults to idle.
    fn busy(&self) -> bool {
        false
    }

    /// Width in pixels of an app-owned widget hosted in the titlebar, right
    /// aligned inside it. Zero, the default, means the app owns no titlebar
    /// widget and the frame keeps the whole bar. A non-zero width moves the
    /// centred title clear of the widget, hands `paint_accessory` a sub-buffer
    /// over it, and routes pointer events landing inside it to
    /// `on_accessory_event` instead of starting a window drag. Keyboard events
    /// are unaffected and keep arriving through `on_event`, so an accessory
    /// that takes text tracks its own focus.
    fn titlebar_accessory_w(&self) -> u32 {
        0
    }

    fn paint_accessory(&mut self, _fb: &mut PaintBuffer) {}

    fn on_accessory_event(&mut self, _event: InputEvent) -> EventOutcome {
        EventOutcome::Idle
    }
}
