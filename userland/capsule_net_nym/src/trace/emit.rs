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

use nonos_libc::mk_debug;

use super::write::Line;

/// Report a step that reached a state worth naming.
pub fn say(stage: &[u8]) {
    let mut line = Line::new(stage);
    let (bytes, len) = line.finish();
    mk_debug(bytes.as_ptr(), len);
}

/// Report a step with the one number that explains it.
pub fn say_num(stage: &[u8], value: u64) {
    let mut line = Line::new(stage);
    line.num(value);
    let (bytes, len) = line.finish();
    mk_debug(bytes.as_ptr(), len);
}

/// Report a step with two numbers, for the cases where one does not say
/// enough on its own: a count against a total, or a length against a limit.
pub fn say_two(stage: &[u8], first: u64, second: u64) {
    let mut line = Line::new(stage);
    line.num(first);
    line.num(second);
    let (bytes, len) = line.finish();
    mk_debug(bytes.as_ptr(), len);
}

/// Report a step alongside a slice of text something else produced.
///
/// The text is not ours, so it is written as bytes rather than interpreted.
/// Control characters are replaced: a gateway is free to send anything, and
/// one that sent an escape sequence would otherwise be writing to the log.
pub fn say_text(stage: &[u8], body: &[u8]) {
    let mut line = Line::new(stage);
    line.text(b" ");
    for &b in body {
        line.text(&[if (0x20..0x7f).contains(&b) { b } else { b'.' }]);
    }
    let (bytes, len) = line.finish();
    mk_debug(bytes.as_ptr(), len);
}
