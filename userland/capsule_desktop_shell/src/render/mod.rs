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
mod cap_names;
pub mod chrome;
pub mod consent;
pub mod desktop_icons;
pub mod desktop_menu;
pub mod fill;
mod icons;
pub mod launchpad;
pub mod layout;
pub mod measure_aa;
pub mod menubar_menu;
pub mod palette;
pub mod panel;
pub mod pkg_consent;
pub mod surface;
pub mod text_aa;
pub mod toasts;
pub mod topbar;
pub mod ui_font;

pub use bottom_taskbar::paint_bottom_taskbar;
pub use chrome::paint_chrome;
pub use icons::{draw_app_glyph, draw_app_icon, draw_tool_icon};
pub use layout::{menubar_rect, spotlight_rect};
pub use toasts::sync_toast_layer;
