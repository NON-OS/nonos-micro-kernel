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

//! Decoded frames on a 1200 by 800 pad, for the gesture engine's tests.

use crate::hid::TouchSample;

pub(super) fn frame(
    x: u32,
    y: u32,
    tip: bool,
    confidence: bool,
    contacts: u32,
    button: bool,
) -> TouchSample {
    TouchSample { x, y, x_max: 1200, y_max: 800, tip, contacts, button, confidence }
}

/// One confident finger down.
pub(super) fn finger(x: u32, y: u32) -> TouchSample {
    frame(x, y, true, true, 1, false)
}

/// Two fingers down, reported at one position.
pub(super) fn two(x: u32, y: u32) -> TouchSample {
    frame(x, y, true, true, 2, false)
}

/// A touch the pad does not trust: the confidence bit clear.
pub(super) fn palm(x: u32, y: u32) -> TouchSample {
    frame(x, y, true, false, 1, false)
}

/// The click button alone, nothing on the surface.
pub(super) fn button(down: bool) -> TouchSample {
    frame(300, 300, false, true, 0, down)
}

pub(super) const LIFTED: TouchSample = TouchSample {
    x: 0,
    y: 0,
    x_max: 1200,
    y_max: 800,
    tip: false,
    contacts: 0,
    button: false,
    confidence: true,
};
