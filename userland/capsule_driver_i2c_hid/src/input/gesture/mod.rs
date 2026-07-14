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

mod on_touch;
mod types;

pub use types::{TouchActions, TouchGesture};
