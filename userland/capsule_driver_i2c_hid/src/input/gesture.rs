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

//! Turn a stream of touch samples into pointer gestures: relative cursor
//! motion with speed-dependent acceleration under one finger, two-finger
//! scroll, and palm suppression via the PTP confidence bit. Clicks come only
//! from the physical clickpad button; taps deliberately do nothing, so a
//! brush of the pad can never click. Motion is relative, not absolute: a
//! laptop pad is a motion surface, and absolute mapping would teleport the
//! cursor to wherever the finger lands. Pure state so it can be exercised by
//! host tests without a device.

/// Nominal width, in screen-pixel-like units, that one edge-to-edge pad
/// traversal covers at gain 1.0. Deliberately conservative: the router
/// stacks a user-policy sensitivity of up to 2.0x on top of this, so the
/// driver curve must stay controllable even at that ceiling.
const NOMINAL_WIDTH: i64 = 800;
/// Acceleration curve in 1/16 gain steps: floor for slow precise motion,
/// one step per SPEED_DIV normalized pixels of per-report speed, and a low
/// ceiling so a flick crosses the screen without becoming untrackable.
const GAIN_FLOOR_X16: i64 = 8;
const GAIN_SPEED_DIV: i64 = 3;
const GAIN_CEIL_X16: i64 = 22;
/// Hard per-report output cap. Whatever the math says, no single report may
/// jump the cursor further than this; it bounds torn-read spikes and keeps a
/// runaway curve physically manageable.
const MOTION_CAP: i32 = 36;
/// Contact-continuity bound: a real finger cannot cross more than this
/// fraction of the pad between two consecutive reports. Anything larger is a
/// torn read or a contact swap, and emitting it would teleport the cursor.
const CONTINUITY_DIV: i64 = 8;

#[derive(Default)]
pub struct TouchGesture {
    was_tip: bool,
    was_button: bool,
    /// Consecutive samples disagreeing with `was_button`, for debounce.
    button_run: u8,
    scrolling: bool,
    scroll_y: u32,
    multi_touch: bool,
    palm: bool,
    last_x: i64,
    last_y: i64,
    // Sub-pixel numerators carried between reports so slow motion below one
    // output pixel per report is accumulated instead of truncated away.
    acc_x: i64,
    acc_y: i64,
}

/// What one sample should do. The driver posts these as input events.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct TouchActions {
    /// Relative cursor motion in nominal screen pixels, already accelerated.
    pub motion: Option<(i32, i32)>,
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
        confidence: bool,
        contacts: u32,
        button: bool,
    ) -> TouchActions {
        let mut act = TouchActions::default();

        // A physical clickpad press maps to the left button, palm or not: a
        // deliberate press is a deliberate press. Debounced: two consecutive
        // samples must agree before an edge is emitted, so a single-frame bit
        // flip (torn read, misplaced field) can never click.
        if button != self.was_button {
            self.button_run = self.button_run.saturating_add(1);
            if self.button_run >= 2 {
                if button {
                    act.button_down = true;
                } else {
                    act.button_up = true;
                }
                self.was_button = button;
                self.button_run = 0;
            }
        } else {
            self.button_run = 0;
        }

        // Degenerate maxima mean the descriptor's Logical Maximum did not
        // parse; motion math through them is meaningless, so only clicks pass.
        if x_max <= 1 || y_max <= 1 {
            return act;
        }
        let x_max = x_max as u32;
        let y_max = y_max as u32;
        // Torn or short polled reports can carry coordinates beyond the pad's
        // declared range; clamp before any delta or tap math.
        let x = x.min(x_max);
        let y = y.min(y_max);

        // Palm suppression: once a non-confident contact is seen, hold all
        // motion until every contact has lifted. Typing with a palm resting
        // on the pad must not steer the cursor.
        if tip && !confidence {
            self.palm = true;
        }
        if self.palm {
            if !tip && contacts == 0 {
                self.palm = false;
            }
            self.was_tip = false;
            return act;
        }

        if contacts >= 2 {
            self.multi_touch = true;
            let scroll_step = (y_max / 96).max(1);
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
            // Two fingers never move the cursor.
            self.was_tip = false;
            return act;
        }
        self.scrolling = false;

        // PTP hybrid reporting alternates which contact a report carries, and
        // only the first report of a frame set carries the true contact count.
        // While several fingers were down, the single-contact frames in
        // between alternate finger positions and would teleport the cursor;
        // hold all movement until every finger has lifted.
        if self.multi_touch {
            if !tip {
                self.multi_touch = false;
            }
            self.was_tip = false;
            return act;
        }

        if tip {
            if self.was_tip {
                let dxp = i64::from(x) - self.last_x;
                let dyp = i64::from(y) - self.last_y;
                let den = i64::from(x_max);
                // Discontinuity: re-anchor silently instead of steering the
                // cursor with a torn coordinate. The next clean report
                // resumes motion from here.
                if dxp.abs() > den / CONTINUITY_DIV || dyp.abs() > den / CONTINUITY_DIV {
                    self.last_x = i64::from(x);
                    self.last_y = i64::from(y);
                    self.acc_x = 0;
                    self.acc_y = 0;
                    self.was_tip = tip;
                    return act;
                }
                // Speed in nominal pixels per report drives the gain; both
                // axes normalize against the X range so the pad's physical
                // aspect ratio is preserved.
                let speed = (dxp.abs() + dyp.abs()) * NOMINAL_WIDTH / den;
                let gain = (GAIN_FLOOR_X16 + speed / GAIN_SPEED_DIV).min(GAIN_CEIL_X16);
                let mx = step_axis(&mut self.acc_x, dxp, gain, den).clamp(-MOTION_CAP, MOTION_CAP);
                let my = step_axis(&mut self.acc_y, dyp, gain, den).clamp(-MOTION_CAP, MOTION_CAP);
                if mx != 0 || my != 0 {
                    act.motion = Some((mx, my));
                }
            } else {
                self.acc_x = 0;
                self.acc_y = 0;
            }
            self.last_x = i64::from(x);
            self.last_y = i64::from(y);
        }
        self.was_tip = tip;
        act
    }
}

// One axis of the accelerated pad-to-pixel conversion. The accumulator holds
// the remainder in (pad-range * 16) denominator units, so slow motion that
// rounds to zero pixels in one report still adds up across reports.
fn step_axis(acc: &mut i64, d_pad: i64, gain_x16: i64, den_pad: i64) -> i32 {
    let den = den_pad * 16;
    *acc += d_pad * NOMINAL_WIDTH * gain_x16;
    let out = *acc / den;
    *acc -= out * den;
    out as i32
}
