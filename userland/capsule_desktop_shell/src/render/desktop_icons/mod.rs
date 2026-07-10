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

//! The VFS root drawn as a grid of file and folder icons on the desktop. Icons
//! fill top to bottom in each column, then wrap to the next column, staying
//! clear of the bottom dock the way a real desktop does.

mod cell_rect;
mod hit;
mod metrics;
mod paint;
mod rows;
mod slot;
mod top;

pub use hit::hit;
pub use paint::paint_desktop_icons;
