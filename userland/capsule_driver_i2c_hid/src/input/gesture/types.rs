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

/// The absolute range the router expects for a POINTER_ABS event.
pub const ABS_RANGE: u32 = 0x7FFF;

#[derive(Default)]
pub struct TouchGesture {
    pub(super) was_tip: bool,
    pub(super) was_button: bool,
    pub(super) down_x: u32,
    pub(super) down_y: u32,
    pub(super) moved: bool,
    pub(super) scrolling: bool,
    pub(super) scroll_y: u32,
    pub(super) multi_touch: bool,
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
