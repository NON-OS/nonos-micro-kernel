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

//! `socket` and `connect`, over net.sockets.

use alloc::vec::Vec;

use super::ops::{DOMAIN, KIND_MIXNET, OP_SOCKET};

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::call::call;

const AF_INET: u64 = 2;
const SOCK_STREAM: u64 = 1;
const SOCK_DGRAM: u64 = 2;
/// Linux ORs these into the type; neither changes what is opened here.
const TYPE_MASK: u64 = 0xFF;

pub fn socket(guest: &mut Guest, family: u64, kind: u64) -> u64 {
    if family != AF_INET {
        return errno::fail(errno::EAFNOSUPPORT);
    }
    /*
     * A guest's stream goes over the mixnet, never the open network, and it
     * holds no capability that could name a socket: there is no second route
     * to disable and no firewall rule to remove.
     */
    let want = match kind & TYPE_MASK {
        SOCK_STREAM => KIND_MIXNET,
        SOCK_DGRAM => return super::dns::open(guest),
        _ => return errno::fail(errno::ENOSYS),
    };
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&DOMAIN.to_le_bytes());
    body.extend_from_slice(&want.to_le_bytes());
    let Some((status, out)) = call(OP_SOCKET, &body, 8) else {
        return errno::fail(errno::EIO);
    };
    if status != 0 || out.len() < 4 {
        return errno::fail(errno::ENOMEM);
    }
    let handle = u32::from_le_bytes([out[0], out[1], out[2], out[3]]);
    match crate::linux::file::install(guest, Fd::socket(handle)) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
