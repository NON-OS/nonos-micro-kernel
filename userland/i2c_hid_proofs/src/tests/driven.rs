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

//! A bound driver being fed frames through the pad's input register.

use nonos_i2cmodel::touchpad::{touch_report, Touch};
use nonos_libc::take_events;

use super::fixture::{rig, Rig};
use crate::input::poll;
use crate::setup;
use crate::state::State;

/// The driver after setup, with whatever setup posted already discarded.
pub(super) fn bound() -> (Rig, State) {
    let r = rig();
    let state = setup::run().expect("setup");
    take_events();
    (r, state)
}

/// Queue one frame at the pad and let the driver poll it.
pub(super) fn frame(r: &Rig, state: &mut State, touch: Touch) {
    r.pad.lock().push_input(&touch_report(&touch));
    poll(state);
}
