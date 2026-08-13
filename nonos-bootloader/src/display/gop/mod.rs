// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod draw;
mod init;
mod read;
mod report;
pub mod state;

pub use draw::{clear_screen, draw_rect, fill_rect, hline, put_pixel, vline};
pub use init::init_gop;
pub use read::get_pixel;
pub use report::report_gop_mode;
pub use state::{
    get_dimensions, get_stride, is_initialized, latched_linear_fb, shutdown_for_exit,
};
