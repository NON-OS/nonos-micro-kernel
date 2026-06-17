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

use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use spin::Mutex;

use crate::dns::Cache;

pub const DNS_PORT: u16 = 53;
pub const DEFAULT_UPSTREAM: [u8; 4] = [1, 1, 1, 1];

pub static CACHE: Mutex<Cache> = Mutex::new(Cache::new());
pub static UDP_PORT: AtomicU32 = AtomicU32::new(0);
static LOCAL_PORT: AtomicU16 = AtomicU16::new(0);
static UPSTREAM: Mutex<[u8; 4]> = Mutex::new(DEFAULT_UPSTREAM);

fn rand_u16() -> Option<u16> {
    let mut b = [0u8; 2];
    if nonos_libc::crypto_random(b.as_mut_ptr(), 2) != 2 {
        return None;
    }
    Some(u16::from_le_bytes(b))
}

pub fn udp_port() -> u32 {
    UDP_PORT.load(Ordering::Acquire)
}

pub fn set_udp_port(port: u32) {
    UDP_PORT.store(port, Ordering::Release);
}

pub fn local_port() -> Option<u16> {
    let current = LOCAL_PORT.load(Ordering::Acquire);
    if current != 0 {
        return Some(current);
    }
    let port = 49152u16 + (rand_u16()? as u32 % 16384) as u16;
    match LOCAL_PORT.compare_exchange(0, port, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => Some(port),
        Err(existing) => Some(existing),
    }
}

pub fn next_xid() -> Option<u16> {
    rand_u16()
}

pub fn now_ms() -> u64 {
    let raw = nonos_libc::mk_time_millis();
    if raw < 0 {
        return 0;
    }
    raw as u64
}

pub fn upstream() -> [u8; 4] {
    *UPSTREAM.lock()
}

pub fn set_upstream(addr: [u8; 4]) {
    *UPSTREAM.lock() = addr;
}
