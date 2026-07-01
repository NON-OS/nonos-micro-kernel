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

use crate::iface::dhcp::fill_marker;

pub fn emit_lease_marker(ip: [u8; 4], prefix: u8, gw: [u8; 4]) {
    let mut buf = [0u8; 64];
    let n = fill_marker::fill_marker(&mut buf, b"[NET-CORE] lease ", ip, prefix, gw);
    mk_debug(buf.as_ptr(), n);
}
