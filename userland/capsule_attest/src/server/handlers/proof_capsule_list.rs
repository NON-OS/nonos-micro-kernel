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

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;

const KNOWN_CAPSULES: &[(&[u8], u64)] = &[
    (b"ramfs", 0x19),
    (b"vfs", 0x19),
    (b"keyring", 0x19),
    (b"entropy", 0x19),
    (b"crypto", 0x19),
    (b"market", 0x19),
    (b"clipboard", 0x19),
    (b"attest", 0x19),
    (b"input_router", 0x19),
    (b"wm", 0x19),
    (b"compositor", 0x7819),
    (b"desktop_shell", 0x1819),
    (b"wallpaper", 0x1819),
    (b"about", 0x1819),
    (b"calculator", 0x1819),
    (b"terminal", 0x1819),
    (b"driver.virtio_gpu0", 0x1F9119),
];

pub fn run(out: &mut [u8], req: &Request) -> usize {
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 4 > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    out[dst..dst + 4].copy_from_slice(&(KNOWN_CAPSULES.len() as u32).to_le_bytes());
    let mut written = 4usize;
    for (name, mask) in KNOWN_CAPSULES {
        let needed = 4 + name.len() + 8;
        if dst + written + needed > out.len() {
            return respond::status(out, req, E_INVAL);
        }
        out[dst + written..dst + written + 4].copy_from_slice(&(name.len() as u32).to_le_bytes());
        out[dst + written + 4..dst + written + 4 + name.len()].copy_from_slice(name);
        out[dst + written + 4 + name.len()..dst + written + 4 + name.len() + 8]
            .copy_from_slice(&mask.to_le_bytes());
        written += needed;
    }
    respond::with_payload(out, req, 0, written)
}
