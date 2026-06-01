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

pub fn announce_live_gui(label: &[u8]) {
    let n = core::cmp::min(label.len(), 160);
    let mut buf = [0u8; 173];
    buf[..11].copy_from_slice(b"[LIVE-GUI] ");
    buf[11..11 + n].copy_from_slice(&label[..n]);
    buf[11 + n] = b'\n';
    let _ = mk_debug(buf.as_ptr(), 12 + n);
}
