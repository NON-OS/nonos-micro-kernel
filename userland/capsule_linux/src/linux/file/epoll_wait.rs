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

//! `epoll_wait`: which of the watched descriptors are ready now.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};
use crate::linux::net::ready;

use super::epoll::EVENT_LEN;

pub fn epoll_wait(guest: &mut Guest, ep: u64, out: u64, max: u64) -> u64 {
    let Some(list) = guest.fds.get(ep as usize).filter(|f| f.kind == Kind::Epoll) else {
        return errno::fail(errno::EBADF);
    };
    let watch = list.watch.clone();
    let mut blob: Vec<u8> = Vec::new();
    let mut hits = 0u64;
    for (fd, wanted, data) in watch {
        if hits >= max {
            break;
        }
        let live = u32::from(ready(guest, fd)) & wanted;
        if live == 0 {
            continue;
        }
        blob.extend_from_slice(&live.to_le_bytes());
        blob.extend_from_slice(&data.to_le_bytes());
        hits += 1;
    }
    if blob.is_empty() {
        return errno::ok(0);
    }
    if guest.write(out, &blob) < blob.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    let _ = EVENT_LEN;
    errno::ok(hits)
}
