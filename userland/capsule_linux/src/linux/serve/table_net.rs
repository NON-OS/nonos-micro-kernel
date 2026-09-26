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

//! Calls that name a socket.

use crate::linux::abi::{nr, nr_path as np};
use crate::linux::call;
use crate::linux::guest::Guest;
use crate::linux::net;
use crate::linux::unix::{self, is_unix};

pub fn net_ops(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Option<u64> {
    let _ = tid;
    Some(match nr {
        nr::SOCKET if a[0] == 1 => unix::socket(guest, a[1]),
        nr::SOCKET => net::socket(guest, a[0], a[1]),
        nr::CONNECT if is_unix(guest, a[0]) => unix::connect(guest, a[0], a[1], a[2]),
        nr::CONNECT => net::connect(guest, a[0], a[1], a[2]),
        nr::SENDTO => net::sendto(guest, a[0], a[1], a[2], a[4], a[5]),
        nr::RECVFROM => net::recvfrom(guest, a[0], a[1], a[2], a[4], a[5]),
        nr::POLL => net::poll(guest, a[0], a[1]),
        np::SELECT | np::PSELECT6 => net::select(guest, a[0], a[1], a[2]),
        np::PPOLL => net::poll(guest, a[0], a[1]),
        nr::SHUTDOWN => call::close(guest, a[0]),
        nr::SENDMSG => unix::sendmsg(guest, a[0], a[1]),
        nr::RECVMSG => unix::recvmsg(guest, a[0], a[1]),
        _ => return None,
    })
}
