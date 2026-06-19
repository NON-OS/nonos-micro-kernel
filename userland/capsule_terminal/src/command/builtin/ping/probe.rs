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

use nonos_libc::{mk_service_lookup, mk_time_millis, mk_yield};

use super::poll::poll_once;
use super::send::send_echo;
use super::{DEADLINE_MS, IP_SERVICE};

pub enum Probe {
    Reply(u64),
    NoRoute,
    NotReady,
    Unreachable,
    Timeout,
    SendFailed,
}

const E_OK: u16 = 0;
const E_NO_CONFIG: u16 = 5;
const E_NO_ROUTE: u16 = 6;
const E_NO_NEIGHBOUR: u16 = 7;

pub fn lookup_ip_service() -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(IP_SERVICE.as_ptr(), IP_SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return None;
    }
    Some(port)
}

pub fn probe(port: u32, dst: [u8; 4]) -> Probe {
    let t0 = mk_time_millis();
    let mut sent = false;
    loop {
        if mk_time_millis().wrapping_sub(t0) > DEADLINE_MS {
            return if sent { Probe::Timeout } else { Probe::Unreachable };
        }
        if !sent {
            match send_echo(port, dst) {
                E_OK => sent = true,
                E_NO_NEIGHBOUR => {}
                E_NO_ROUTE => return Probe::NoRoute,
                E_NO_CONFIG => return Probe::NotReady,
                _ => return Probe::SendFailed,
            }
        }
        if let Some(rtt) = poll_once(port, dst, t0) {
            return Probe::Reply(rtt);
        }
        mk_yield();
    }
}
