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
static LOCAL_PORT: AtomicU16 = AtomicU16::new(53535);
static XID: AtomicU16 = AtomicU16::new(0x3011);
static UPSTREAM: Mutex<[u8; 4]> = Mutex::new(DEFAULT_UPSTREAM);

pub fn udp_port() -> u32 {
    UDP_PORT.load(Ordering::Acquire)
}

pub fn set_udp_port(port: u32) {
    UDP_PORT.store(port, Ordering::Release);
}

pub fn local_port() -> u16 {
    LOCAL_PORT.load(Ordering::Acquire)
}

pub fn next_xid() -> u16 {
    XID.fetch_add(1, Ordering::AcqRel).wrapping_add(1)
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
