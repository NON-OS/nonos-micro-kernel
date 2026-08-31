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

const LOUD_ATTEMPTS: u32 = 3;
const QUIET_PERIOD: u32 = 64;

pub fn up() {
    emit(b"[NBLK] up", "");
}

pub fn setup_fail(attempt: u32, err: &str) {
    if attempt >= LOUD_ATTEMPTS && attempt % QUIET_PERIOD != 0 {
        return;
    }
    emit(b"[NBLK] setup-fail ", err);
}

fn emit(tag: &[u8], detail: &str) {
    let mut line = [0u8; 128];
    let mut n = 0usize;
    for b in tag.iter().chain(detail.as_bytes()).chain(b"\n") {
        if n == line.len() {
            break;
        }
        line[n] = *b;
        n += 1;
    }
    let _ = mk_debug(line.as_ptr(), n);
}
