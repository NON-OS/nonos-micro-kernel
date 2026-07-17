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

use crate::iface::dhcp::{write_decimal_u8, write_octet_quad};
use crate::server::handlers::dhcp_status::encode_body;

pub fn emit_status_selfcheck() {
    let mut body = [0u8; 22];
    encode_body(&mut body);
    let state_code = body[0];
    let ip = [body[1], body[2], body[3], body[4]];
    let mut buf = [0u8; 64];
    let mut pos = 0usize;
    for &b in b"[NET-CORE] lease-status state=" {
        buf[pos] = b;
        pos += 1;
    }
    pos = write_decimal_u8::write_decimal_u8(&mut buf, pos, state_code);
    for &b in b" ip=" {
        buf[pos] = b;
        pos += 1;
    }
    pos = write_octet_quad::write_octet_quad(&mut buf, pos, ip);
    buf[pos] = b'\n';
    mk_debug(buf.as_ptr(), pos + 1);
}
