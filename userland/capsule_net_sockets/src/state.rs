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

use nonos_libc::mk_service_lookup;

static TCP: AtomicU32 = AtomicU32::new(0);
static UDP: AtomicU32 = AtomicU32::new(0);
static NYM: AtomicU32 = AtomicU32::new(0);
static DNS: AtomicU32 = AtomicU32::new(0);

pub fn tcp() -> u32 {
    TCP.load(Ordering::Acquire)
}

pub fn udp() -> u32 {
    UDP.load(Ordering::Acquire)
}

pub fn nym() -> u32 {
    let port = NYM.load(Ordering::Acquire);
    if port != 0 {
        return port;
    }
    let port = lookup(b"net.nym").unwrap_or(0);
    if port != 0 {
        NYM.store(port, Ordering::Release);
    }
    port
}

pub fn dns() -> u32 {
    DNS.load(Ordering::Acquire)
}

pub fn discover() -> Result<(), ()> {
    TCP.store(lookup(b"net.tcp")?, Ordering::Release);
    UDP.store(lookup(b"net.udp")?, Ordering::Release);
    DNS.store(lookup(b"net.dns")?, Ordering::Release);
    NYM.store(lookup(b"net.nym").unwrap_or(0), Ordering::Release);
    Ok(())
}

fn lookup(name: &[u8]) -> Result<u32, ()> {
    let mut port = 0u32;
    let mut pid = 0u32;
    if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 || port == 0 {
        return Err(());
    }
    Ok(port)
}
