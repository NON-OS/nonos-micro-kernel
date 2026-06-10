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
mod close_button_rect;
mod draw_close_button;
mod draw_extra;
mod extra_rects;
mod fill_box;
mod plot;
mod types;

pub use close_button_rect::close_button_rect;
pub use draw_close_button::draw_close_button;
pub use draw_extra::{draw_maximize_button, draw_minimize_button};
pub use extra_rects::{maximize_button_rect, minimize_button_rect};
