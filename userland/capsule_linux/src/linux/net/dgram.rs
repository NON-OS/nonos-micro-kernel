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

//! `sendto` and `recvfrom`, which differ from write and read only in carrying
//! an address.

use crate::linux::call;
use crate::linux::guest::{Guest, Kind};

use super::addr::inet;
use super::dgram_addr::{encode, fill};
use super::dns;

pub fn sendto(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64, alen: u64) -> u64 {
    if !is_resolver(guest, fd) {
        return call::write(guest, fd, buf, len);
    }
    /*
     * A program with no `resolv.conf` asks the loopback address, and one with
     * a configured nameserver asks that.
     */
    let peer = inet(guest, at, alen).unwrap_or((53, [127, 0, 0, 1]));
    dns::query(guest, fd, buf, len, encode(peer))
}

pub fn recvfrom(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64, alen: u64) -> u64 {
    if !is_resolver(guest, fd) {
        return call::read(guest, fd, buf, len);
    }
    let (got, from) = dns::answer_out(guest, fd, buf, len);
    match from {
        Some(peer) if at != 0 => fill(guest, at, alen, peer, got),
        _ => got,
    }
}

fn is_resolver(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Resolver)
}
