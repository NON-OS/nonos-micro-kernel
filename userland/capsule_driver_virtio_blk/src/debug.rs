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

const PREFIX: &[u8] = b"[virtio_blk] ";
const NEWLINE: &[u8] = b"\n";
const SPACE: &[u8] = b" ";
const MAX_LABEL: usize = 200;

pub fn marker(label: &[u8]) {
    let label_len = if label.len() > MAX_LABEL { MAX_LABEL } else { label.len() };
    let _ = mk_debug(PREFIX.as_ptr(), PREFIX.len());
    let _ = mk_debug(label.as_ptr(), label_len);
    let _ = mk_debug(NEWLINE.as_ptr(), NEWLINE.len());
}

pub fn marker_i64(label: &[u8], value: i64) {
    let label_len = if label.len() > MAX_LABEL { MAX_LABEL } else { label.len() };
    let _ = mk_debug(PREFIX.as_ptr(), PREFIX.len());
    let _ = mk_debug(label.as_ptr(), label_len);
    let _ = mk_debug(SPACE.as_ptr(), SPACE.len());
    emit_i64(value);
    let _ = mk_debug(NEWLINE.as_ptr(), NEWLINE.len());
}

fn emit_i64(value: i64) {
    if value < 0 {
        let _ = mk_debug(b"-".as_ptr(), 1);
        emit_u64((value as i128).unsigned_abs() as u64);
    } else {
        emit_u64(value as u64);
    }
}

fn emit_u64(mut value: u64) {
    if value == 0 {
        let _ = mk_debug(b"0".as_ptr(), 1);
        return;
    }
    let mut buf = [0u8; 20];
    let mut idx = buf.len();
    while value > 0 {
        idx -= 1;
        buf[idx] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    let _ = mk_debug(buf.as_ptr().wrapping_add(idx), buf.len() - idx);
}
