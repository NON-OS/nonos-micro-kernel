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

pub(super) fn trace(pid: u32, label: &[u8], rc: i64) {
    if pid != 0x17 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-CALL] pid=");
    crate::sys::serial::trace_hex(pid as u64);
    crate::sys::serial::trace(b" ");
    crate::sys::serial::trace(label);
    crate::sys::serial::trace(b" rc=");
    if rc < 0 {
        crate::sys::serial::trace(b"-");
        crate::sys::serial::trace_dec((-rc) as u64);
    } else {
        crate::sys::serial::trace_dec(rc as u64);
    }
    crate::sys::serial::traceln(b"");
}
