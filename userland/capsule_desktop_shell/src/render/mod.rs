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

mod bottom_taskbar;
pub mod chrome;
pub mod fill;
pub mod layout;
mod side_launchers;
mod text;

pub use bottom_taskbar::paint_bottom_taskbar;
pub use chrome::paint_chrome;
pub use side_launchers::paint_side_launchers;
pub use text::draw_overlay_text;
pub use layout::{menubar_rect, spotlight_rect};
