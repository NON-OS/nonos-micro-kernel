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

use super::render::write_line;
use crate::sys::serial;

const TAG_OK: u32 = 0x0000FF00;
const TAG_WARN: u32 = 0x0000AAFF;
const TAG_ERR: u32 = 0x000000FF;
const TAG_INFO: u32 = 0x00FFCC00;
const TAG_STAGE: u32 = 0x00AAAAAA;

pub fn info(msg: &str) {
    serial::print(b"[INFO] ");
    serial::print_str(msg);
    serial::print(b"\r\n");
    write_line("INFO", msg, TAG_INFO);
}

pub fn ok(tag: &str, msg: &str) {
    serial::print(b"[");
    serial::print_str(tag);
    serial::print(b"] ");
    serial::print_str(msg);
    serial::print(b"\r\n");
    write_line(tag, msg, TAG_OK);
}

pub fn warn(msg: &str) {
    serial::print(b"[WARN] ");
    serial::print_str(msg);
    serial::print(b"\r\n");
    write_line("WARN", msg, TAG_WARN);
}

pub fn error(msg: &str) {
    serial::print(b"[ERROR] ");
    serial::print_str(msg);
    serial::print(b"\r\n");
    write_line("ERROR", msg, TAG_ERR);
}

pub fn stage(tag: &str, msg: &str) {
    serial::print(b"[");
    serial::print_str(tag);
    serial::print(b"] ");
    serial::print_str(msg);
    serial::print(b"\r\n");
    write_line(tag, msg, TAG_STAGE);
}
