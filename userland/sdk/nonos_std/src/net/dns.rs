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

use alloc::vec::Vec;

use crate::io::{Error, ErrorKind, Result};
use crate::net::addr::Ipv4Addr;

const DNS_NAME: &[u8] = b"net.dns";
const MAGIC: u32 = 0x4E44_4E53;
const VERSION: u16 = 1;
const OP_RESOLVE_A: u16 = 2;
const HDR_LEN: usize = 20;

pub(crate) fn resolve_host(host: &str) -> Result<Ipv4Addr> {
    if host.is_empty() || host.len() > 255 {
        return Err(Error::new(ErrorKind::InvalidInput, "bad hostname"));
    }
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = nonos_libc::mk_service_lookup(DNS_NAME.as_ptr(), DNS_NAME.len(), &mut port, &mut owner);
    if rc != 0 {
        return Err(Error::new(ErrorKind::NotFound, "net.dns unavailable"));
    }
    let tx = frame(host.as_bytes());
    let mut rx = [0u8; HDR_LEN + 4];
    let n = nonos_libc::mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if n < (HDR_LEN + 4) as i64 {
        return Err(Error::new(ErrorKind::Other, "dns resolve failed"));
    }
    if u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return Err(Error::new(ErrorKind::NotFound, "host not found"));
    }
    Ok(Ipv4Addr::new(rx[20], rx[21], rx[22], rx[23]))
}

fn frame(host: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + host.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&OP_RESOLVE_A.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&(host.len() as u32).to_le_bytes());
    f.extend_from_slice(host);
    f
}
