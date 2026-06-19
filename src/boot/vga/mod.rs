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

pub mod colors;
mod output;
mod splash;

pub use output::{
    buffer_size, clear_screen, fill_row, read_char, scroll_up, visual_delay, write_at, write_char,
    write_string, VGA_BUFFER, VGA_HEIGHT, VGA_WIDTH,
};
pub use splash::{show_boot_splash, show_panic, show_progress, show_status_line};
