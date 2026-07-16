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

/// The primary contact and gross state of one touch report.
#[derive(Clone, Copy, Default)]
pub struct TouchSample {
    pub x: u32,
    pub y: u32,
    pub x_max: i32,
    pub y_max: i32,
    /// The primary finger is touching the surface.
    pub tip: bool,
    /// Number of fingers down, for gesture logic.
    pub contacts: u32,
    /// The physical click button (clickpad) is pressed.
    pub button: bool,
    /// The contact is a deliberate finger (PTP confidence bit). True when the
    /// device does not report confidence at all.
    pub confidence: bool,
}
