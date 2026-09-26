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


//! `sendmsg` and `recvmsg` on the display socket.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

/// struct msghdr on x86_64.
const IOV_AT: usize = 16;
const IOVLEN_AT: usize = 24;
const CONTROL_AT: usize = 32;
const CONTROLLEN_AT: usize = 40;
pub const MSGHDR_LEN: usize = 56;

pub struct Msg {
    pub bytes: Vec<u8>,
    pub fds: Vec<u32>,
}

pub fn read_msghdr(guest: &Guest, at: u64) -> Option<Msg> {
    let raw = guest.read(at, MSGHDR_LEN)?;
    let iov = u64le(&raw, IOV_AT);
    let iovlen = u64le(&raw, IOVLEN_AT);
    let control = u64le(&raw, CONTROL_AT);
    let controllen = u64le(&raw, CONTROLLEN_AT);
    let bytes = super::msg_parts::gather(guest, iov, iovlen)?;
    let fds = super::msg_rights::rights(guest, control, controllen);
    Some(Msg { bytes, fds })
}

pub(super) fn u64le(b: &[u8], at: usize) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&b[at..at + 8]);
    u64::from_le_bytes(w)
}
