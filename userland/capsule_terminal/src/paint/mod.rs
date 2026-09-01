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

mod block_chrome;
mod block_meta;
mod compose;
mod constants;
mod draw_cursor;
mod draw_grid;
mod draw_input_line;
mod fetch;
mod fetch_banner;
mod fetch_palette;
mod fetch_uptime;
mod fit_text;
mod footer;
mod header;
mod line_chars;
mod line_text;
mod line_window;
mod metrics;
mod prompt;
mod shade;
mod suggestion;
mod syntax;
pub mod tab_bar;
mod tab_chip;
mod tab_label;
pub mod tab_pill;
pub mod tokens;

pub use compose::paint_tabs;
pub use tab_bar::draw_tab_bar;
