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

use super::print_hex::print_hex_u64;

pub fn emit_fatal_notice(name: &[u8], rip: u64) {
    crate::sys::serial::print(b"[PANIC ");
    crate::sys::serial::print(name);
    crate::sys::serial::print(b"] fatal kernel fault, no recovery path rip=");
    print_hex_u64(rip);
    crate::sys::serial::println(b" -- CPU halting, boot terminated");
}

/// Same notice without the COM1 spinlock, for a fault taken when the
/// interrupted code may already hold it and a deadlock would cost the only
/// diagnosis the machine can still produce.
pub fn emit_fatal_notice_nolock(name: &[u8], rip: u64) {
    emit_raw(b"[PANIC ");
    emit_raw(name);
    emit_raw(b"] fatal kernel fault, no recovery path rip=");
    let hex = b"0123456789abcdef";
    let mut buf = [0u8; 18];
    buf[0] = b'0';
    buf[1] = b'x';
    for i in 0..16 {
        buf[2 + i] = hex[((rip >> ((15 - i) * 4)) & 0xF) as usize];
    }
    emit_raw(&buf);
    emit_raw(b" -- CPU halting, boot terminated\r\n");
}

fn emit_raw(s: &[u8]) {
    for &ch in s {
        crate::sys::serial::core::write_byte(ch);
    }
}
