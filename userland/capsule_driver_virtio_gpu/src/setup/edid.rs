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

//! Ask the panel how big it physically is, once, at boot. Best-effort: a host
//! that never offered EDID, a monitor that declares no size, and a query that
//! the device rejects all leave the scanout path untouched and say so in the
//! boot log, so an assumed DPI is never mistaken for a measured one.

use super::probe_3d::{push, push_dec};
use crate::device::cmd::get_edid;
use crate::device::ControlQueue;

use nonos_libc::mk_debug;

const PRIMARY_SCANOUT: u32 = 0;

pub fn fetch(q: &ControlQueue, edid: bool) {
    if !edid {
        log(b"[GPU-EDID] edid not offered (physical size unknown)");
        return;
    }
    match get_edid(q, PRIMARY_SCANOUT) {
        Ok(Some(size)) => log_size(size.width_cm, size.height_cm),
        Ok(None) => log(b"[GPU-EDID] panel declares no physical size"),
        Err(e) => log_err(b"[GPU-EDID] query failed: ", e),
    }
}

fn log(msg: &[u8]) {
    mk_debug(msg.as_ptr(), msg.len());
}

fn log_err(prefix: &[u8], e: &str) {
    let mut buf = [0u8; 96];
    let mut n = 0;
    push(&mut buf, &mut n, prefix);
    push(&mut buf, &mut n, e.as_bytes());
    mk_debug(buf.as_ptr(), n);
}

fn log_size(width_cm: u32, height_cm: u32) {
    let mut buf = [0u8; 96];
    let mut n = 0;
    push(&mut buf, &mut n, b"[GPU-EDID] panel scanout=0 ");
    push_dec(&mut buf, &mut n, width_cm);
    push(&mut buf, &mut n, b"cm x ");
    push_dec(&mut buf, &mut n, height_cm);
    push(&mut buf, &mut n, b"cm");
    mk_debug(buf.as_ptr(), n);
}
