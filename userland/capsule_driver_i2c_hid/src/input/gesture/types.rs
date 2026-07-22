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

/// Nominal width, in screen-pixel-like units, that one edge-to-edge pad
/// traversal covers at gain 1.0. Deliberately conservative: the router
/// stacks a user-policy sensitivity of up to 2.0x on top of this, so the
/// driver curve must stay controllable even at that ceiling.
pub(super) const NOMINAL_WIDTH: i64 = 800;
/// Acceleration curve in 1/16 gain steps: floor for slow precise motion,
/// one step per SPEED_DIV normalized pixels of per-report speed, and a low
/// ceiling so a flick crosses the screen without becoming untrackable.
pub(super) const GAIN_FLOOR_X16: i64 = 8;
pub(super) const GAIN_SPEED_DIV: i64 = 3;
pub(super) const GAIN_CEIL_X16: i64 = 22;
/// Hard per-report output cap. Whatever the math says, no single report may
/// jump the cursor further than this; it bounds torn-read spikes and keeps a
/// runaway curve physically manageable.
pub(super) const MOTION_CAP: i32 = 36;
/// Contact-continuity bound: a real finger cannot cross more than this
/// fraction of the pad between two consecutive reports. Anything larger is a
/// torn read or a contact swap, and emitting it would teleport the cursor.
pub(super) const CONTINUITY_DIV: i64 = 8;
/// Tap-to-click: a single finger that touches down and lifts within this many
/// reports, having travelled less than 1/`TAP_TRAVEL_DIV` of the pad, is a tap
/// and emits a left click. Generous frame bound so a tap registers across pad
/// report rates; the travel bound is what actually separates a tap from a drag.
pub(super) const TAP_MAX_FRAMES: u16 = 40;
pub(super) const TAP_TRAVEL_DIV: i64 = 16;

#[derive(Default)]
pub struct TouchGesture {
    pub(super) was_tip: bool,
    pub(super) was_button: bool,
    /// Consecutive samples disagreeing with `was_button`, for debounce.
    pub(super) button_run: u8,
    pub(super) scrolling: bool,
    pub(super) scroll_y: u32,
    pub(super) multi_touch: bool,
    pub(super) palm: bool,
    pub(super) last_x: i64,
    pub(super) last_y: i64,
    // Sub-pixel numerators carried between reports so slow motion below one
    // output pixel per report is accumulated instead of truncated away.
    pub(super) acc_x: i64,
    pub(super) acc_y: i64,
    // Tap-to-click tracking for the current single-finger touch: how many
    // reports the finger has been down, how far it has travelled, and whether
    // the touch is still eligible to become a tap (a physical press or a
    // palm/multi-touch cancels it).
    pub(super) tip_frames: u16,
    pub(super) tap_travel: i64,
    pub(super) tap_ok: bool,
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
