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

use alloc::{vec, vec::Vec};

use super::wire::{errno, frame, BODY_OFF, HDR_LEN};
use crate::io::{Error, ErrorKind, Result};

const SOCKETS_NAME: &[u8] = b"net.sockets";

pub(crate) fn port() -> Result<u32> {
    let mut p = 0u32;
    let mut owner = 0u32;
    let rc =
        nonos_libc::mk_service_lookup(SOCKETS_NAME.as_ptr(), SOCKETS_NAME.len(), &mut p, &mut owner);
    if rc != 0 {
        return Err(Error::new(ErrorKind::NotFound, "net.sockets unavailable"));
    }
    Ok(p)
}

pub(crate) fn call(port: u32, op: u16, body: &[u8], reply_cap: usize) -> Result<Vec<u8>> {
    let tx = frame(op, body);
    let mut rx = vec![0u8; BODY_OFF + reply_cap];
    let rc = nonos_libc::mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < HDR_LEN as i64 {
        return Err(Error::new(ErrorKind::Other, "net.sockets ipc failed"));
    }
    let code = errno(&rx);
    if code != 0 {
        return Err(map_errno(code));
    }
    rx.truncate(rc as usize);
    Ok(rx)
}

fn map_errno(code: u16) -> Error {
    let kind = match code {
        5 => ErrorKind::NotFound,
        8 | 9 => ErrorKind::InvalidInput,
        _ => ErrorKind::Other,
    };
    Error::new(kind, "net.sockets error")
}
