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

//! Host proofs for the window manager's geometry. The `#[path]` includes pull
//! in the real production source so the tests pin the shipping hit-testing and
//! clamping used by click-to-raise and window placement. `super::rect::Rect`
//! in the constrain module resolves to the sibling include below.

// Included flat at the crate root: constrain says `use super::rect::Rect`, and
// from a root-level module `super` is the crate root, where `rect` lives.
#[path = "../../capsule_wm/src/geometry/rect.rs"]
pub mod rect;

#[path = "../../capsule_wm/src/geometry/constrain.rs"]
pub mod constrain;

#[cfg(test)]
mod geometry_tests;
