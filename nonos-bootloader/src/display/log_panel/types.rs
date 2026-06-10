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

pub const MAX_LOG_LINES: usize = 256;
pub const LOG_LINE_LEN: usize = 120;
pub const LINE_HEIGHT: u32 = 16;
pub const LOG_START_Y: u32 = 180;
pub const LOG_START_X: u32 = 40;
pub const BOTTOM_MARGIN: u32 = 50;

pub fn get_log_area() -> (u32, u32) {
    (LOG_START_X, LOG_START_Y)
}

pub fn max_visible_lines() -> usize {
    let (_, height) = crate::display::gop::get_dimensions();
    if height == 0 {
        return MAX_LOG_LINES;
    }
    let available = height.saturating_sub(LOG_START_Y).saturating_sub(BOTTOM_MARGIN);
    let lines = (available / LINE_HEIGHT) as usize;
    lines.min(MAX_LOG_LINES)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Error,
    Security,
}
