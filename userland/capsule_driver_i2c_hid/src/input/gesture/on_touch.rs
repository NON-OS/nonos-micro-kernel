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

use super::types::{TouchActions, TouchGesture, ABS_RANGE};

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

        // A physical clickpad press maps straight to the left button.
        if button && !self.was_button {
            act.button_down = true;
        }
        if !button && self.was_button {
            act.button_up = true;
        }
        self.was_button = button;

        // Degenerate maxima mean the descriptor's Logical Maximum did not
        // parse; scaling through 1 would slam every touch to the far screen
        // edge, so clicks pass through but motion is suppressed.
        if x_max <= 1 || y_max <= 1 {
            return act;
        }
        let x_max = x_max as u32;
        let y_max = y_max as u32;
        // Torn or short polled reports can carry coordinates beyond the pad's
        // range; unclamped they scale past ABS_RANGE and the router rails the
        // cursor to a screen edge where only a one-pixel sliver renders.
        let x = x.min(x_max);
        let y = y.min(y_max);
        let tap_travel = x_max / 20; // a tap must stay within ~5% of the pad
        let scroll_step = (y_max / 60).max(1);

        if contacts >= 2 {
            self.multi_touch = true;
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

        // PTP hybrid reporting alternates which contact a report carries, and
        // only the first report of a frame set carries the true contact count.
        // While several fingers were down, the single-contact frames in between
        // alternate finger positions and would teleport the cursor; hold all
        // movement until every finger has lifted.
        if self.multi_touch {
            if !tip {
                self.multi_touch = false;
            }
            self.was_tip = false;
            return act;
        }

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
            act.move_to = Some((
                (u64::from(x) * u64::from(ABS_RANGE) / u64::from(x_max)) as u32,
                (u64::from(y) * u64::from(ABS_RANGE) / u64::from(y_max)) as u32,
            ));
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
