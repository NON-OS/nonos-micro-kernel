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

// Defined in syscall.S. Installed in LSTAR by manager::init.
extern "C" {
    pub fn syscall_entry_asm();
}

/// Words the entry code saves before it calls the handler, read from the file
/// the entry code itself assembles against, so the two cannot disagree.
pub const SYSCALL_FRAME_WORDS: usize = set_value(include_str!("syscall_frame.inc"));

/*
 * The value after the last comma of the `.set` line. A file this does not
 * parse fails the build here rather than yielding a wrong count.
 */
const fn set_value(text: &str) -> usize {
    let b = text.as_bytes();
    let mut i = b.len();
    while i > 0 && !b[i - 1].is_ascii_digit() {
        i -= 1;
    }
    let end = i;
    while i > 0 && b[i - 1].is_ascii_digit() {
        i -= 1;
    }
    assert!(i < end && i > 0 && b[i - 1] == b' ', "syscall_frame.inc: no count");
    let mut n = 0;
    while i < end {
        n = n * 10 + (b[i] - b'0') as usize;
        i += 1;
    }
    n
}
