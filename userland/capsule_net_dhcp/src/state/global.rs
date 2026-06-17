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

use core::sync::atomic::{AtomicU32, Ordering};
use spin::Mutex;

use super::lease::Lease;
use crate::dhcp::State as ClientState;

pub struct Global {
    pub l2_port: AtomicU32,
    pub ip_port: AtomicU32,
    pub mac: Mutex<[u8; 6]>,
    pub client_state: Mutex<ClientState>,
    pub lease: Mutex<Lease>,
}

impl Global {
    pub const fn new() -> Self {
        Self {
            l2_port: AtomicU32::new(0),
            ip_port: AtomicU32::new(0),
            mac: Mutex::new([0; 6]),
            client_state: Mutex::new(ClientState::Init),
            lease: Mutex::new(Lease::empty()),
        }
    }

    pub fn next_xid(&self) -> Option<u32> {
        let v = rand_u32()?;
        Some(if v == 0 { 1 } else { v })
    }

    pub fn l2(&self) -> u32 {
        self.l2_port.load(Ordering::Acquire)
    }

    pub fn ip(&self) -> u32 {
        self.ip_port.load(Ordering::Acquire)
    }
}

fn rand_u32() -> Option<u32> {
    let mut b = [0u8; 4];
    let n = nonos_libc::crypto_random(b.as_mut_ptr(), 4);
    if n != 4i64 {
        return None;
    }
    Some(u32::from_le_bytes(b))
}

pub static STATE: Global = Global::new();
