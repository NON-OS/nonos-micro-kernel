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

//! Turn a stream of absolute touch samples into pointer gestures: move the
//! cursor under one finger, click on a tap (finger down then up without
//! travel), and scroll with two fingers. Pure state so it can be exercised by
//! host tests without a device.

/// The absolute range the router expects for a POINTER_ABS event.
pub const ABS_RANGE: u32 = 0x7FFF;

#[derive(Default)]
pub struct TouchGesture {
    was_tip: bool,
    was_button: bool,
    down_x: u32,
    down_y: u32,
    moved: bool,
    scrolling: bool,
    scroll_y: u32,
}

/// What one sample should do. The driver posts these as input events.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct TouchActions {
    /// Absolute cursor position, already scaled to 0..ABS_RANGE.
    pub move_to: Option<(u32, u32)>,
    pub wheel: i32,
    pub button_down: bool,
    pub button_up: bool,
}

impl TouchGesture {
    #[allow(clippy::too_many_arguments)]
    pub fn on_touch(
        &mut self,
        x: u32,
        y: u32,
        x_max: i32,
        y_max: i32,
        tip: bool,
        contacts: u32,
        button: bool,
    ) -> TouchActions {
        let mut act = TouchActions::default();
        let x_max = x_max.max(1) as u32;
        let y_max = y_max.max(1) as u32;
        let tap_travel = x_max / 20; // a tap must stay within ~5% of the pad
        let scroll_step = (y_max / 60).max(1);

        // A physical clickpad press maps straight to the left button.
        if button && !self.was_button {
            act.button_down = true;
        }
        if !button && self.was_button {
            act.button_up = true;
        }
        self.was_button = button;

        if contacts >= 2 {
            if self.scrolling {
                let dy = y as i32 - self.scroll_y as i32;
                if dy.unsigned_abs() >= scroll_step {
                    act.wheel = dy / scroll_step as i32;
                    self.scroll_y = y;
                }
            } else {
                self.scrolling = true;
                self.scroll_y = y;
            }
            // Two fingers never move the cursor or leave a pending tap.
            self.was_tip = false;
            return act;
        }
        self.scrolling = false;

        if tip {
            if self.was_tip {
                let dx = (x as i32 - self.down_x as i32).unsigned_abs();
                let dy = (y as i32 - self.down_y as i32).unsigned_abs();
                if dx > tap_travel || dy > tap_travel {
                    self.moved = true;
                }
            } else {
                self.down_x = x;
                self.down_y = y;
                self.moved = false;
            }
            act.move_to = Some((x * ABS_RANGE / x_max, y * ABS_RANGE / y_max));
        } else if self.was_tip && !self.moved {
            // Finger lifted without travelling: a tap, delivered as a click at
            // the cursor's current position.
            act.button_down = true;
            act.button_up = true;
        }
        self.was_tip = tip;
        act
    }
}
