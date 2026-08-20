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

#![no_std]

extern crate alloc;

pub mod animation;
pub mod component_dispatch;
pub mod components;
pub mod decorations;
pub mod design;
pub mod font;
pub mod image;
pub mod paint;
pub mod protocol;
pub mod qr;
pub mod server;
pub mod theme;

pub use animation::{easing, runner, state as animation_state, timing, transitions};
pub use components::{
    badge, button, card, checkbox, colorpicker, datepicker, dropdown, glass_panel, input, label,
    list, menu, progress, radio, scroll, slider, statusbar, tabbar, toggle, tooltip,
};
pub use design::{border, color, shadow, spacing, typography};
pub use font::{atlas, glyph, render as font_render, ttf};
pub use image::{bmp, gif, jpeg, lz4_raw, png, types};
pub use qr::{ecc, format, mask, place, render as qr_render};
