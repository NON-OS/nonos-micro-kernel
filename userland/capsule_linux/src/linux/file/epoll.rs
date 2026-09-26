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

//! `epoll_create1` and `epoll_ctl`: the interest list a program keeps.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::slot::install;

const EPOLL_CTL_ADD: u64 = 1;
const EPOLL_CTL_DEL: u64 = 2;
const EPOLL_CTL_MOD: u64 = 3;

/// `struct epoll_event` is packed on x86_64: a u32 of events then a u64
/// of caller data, twelve bytes and not sixteen.
pub const EVENT_LEN: usize = 12;

pub fn epoll_create(guest: &mut Guest) -> u64 {
    match install(guest, Fd::empty(Kind::Epoll)) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

pub fn epoll_ctl(guest: &mut Guest, ep: u64, op: u64, fd: u64, event: u64) -> u64 {
    let entry = match op {
        EPOLL_CTL_DEL => None,
        EPOLL_CTL_ADD | EPOLL_CTL_MOD => match read_event(guest, event) {
            Some(pair) => Some(pair),
            None => return errno::fail(errno::EFAULT),
        },
        _ => return errno::fail(errno::EINVAL),
    };
    let Some(list) = guest.fds.get_mut(ep as usize).filter(|f| f.kind == Kind::Epoll) else {
        return errno::fail(errno::EBADF);
    };
    list.watch.retain(|(f, _, _)| *f != fd);
    if let Some((events, data)) = entry {
        list.watch.push((fd, events, data));
    }
    errno::ok(0)
}

fn read_event(guest: &Guest, at: u64) -> Option<(u32, u64)> {
    let raw = guest.read(at, EVENT_LEN)?;
    let events = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    let mut data = [0u8; 8];
    data.copy_from_slice(&raw[4..12]);
    Some((events, u64::from_le_bytes(data)))
}
