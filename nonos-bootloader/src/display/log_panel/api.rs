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

use super::buffer::{clear_buffer, push_entry};
use super::render::{clear_display, redraw_all, render_after_log};
use super::types::LogLevel;

pub fn log(level: LogLevel, msg: &[u8]) {
    let count = push_entry(level, msg);
    render_after_log(count);
}

pub fn log_info(msg: &[u8]) {
    log(LogLevel::Info, msg);
}

pub fn log_ok(msg: &[u8]) {
    log(LogLevel::Ok, msg);
}

pub fn log_warn(msg: &[u8]) {
    log(LogLevel::Warn, msg);
}

pub fn log_error(msg: &[u8]) {
    log(LogLevel::Error, msg);
}

pub fn log_security(msg: &[u8]) {
    log(LogLevel::Security, msg);
}

pub fn clear() {
    clear_buffer();
    clear_display();
}

pub fn refresh() {
    redraw_all();
}
