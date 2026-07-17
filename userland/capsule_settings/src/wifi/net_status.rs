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

//! Query net_core's DHCP lease. It registers `net.dhcp.client` only after binding
//! an interface, so a failed lookup means the stack bound nothing.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::lease::{Lease, NetStatus};

const NET_DHCP_SERVICE: &[u8] = b"net.dhcp.client";
const MAGIC_NDHC: u32 = 0x4E44_4843;
const OP_LEASE_STATUS: u16 = 3;
const HDR_LEN: usize = 20;
// Generous: a bound net_core spends most of a poll draining frames, so a short
// deadline misses its reply and the panel wrongly reads the stack as absent.
const LEASE_TIMEOUT_MS: u64 = 2500;
const STATE_BOUND: u8 = 3;

/// Query net_core for its lease. `Down` when the service is not registered;
/// otherwise the bound port and any address it holds.
pub fn net_status() -> NetStatus {
    let mut port: u32 = 0;
    let rc = mk_service_lookup(
        NET_DHCP_SERVICE.as_ptr(),
        NET_DHCP_SERVICE.len(),
        &mut port as *mut u32,
        ptr::null_mut(),
    );
    if rc != 0 || port == 0 {
        return NetStatus::NoService;
    }
    let mut req = [0u8; HDR_LEN];
    req[0..4].copy_from_slice(&MAGIC_NDHC.to_le_bytes());
    req[4..6].copy_from_slice(&1u16.to_le_bytes());
    req[6..8].copy_from_slice(&OP_LEASE_STATUS.to_le_bytes());
    let mut resp = [0u8; HDR_LEN + 22];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        LEASE_TIMEOUT_MS,
    );
    if n < (HDR_LEN + 22) as i64 {
        return NetStatus::NoReply;
    }
    decode(&resp[HDR_LEN..])
}

// Decode the 22-byte body: state, address fields, then the bound port.
fn decode(b: &[u8]) -> NetStatus {
    let bound = u32::from_le_bytes([b[18], b[19], b[20], b[21]]);
    if b[0] != STATE_BOUND {
        return NetStatus::Unbound { port: bound };
    }
    let lease = Lease {
        ip: [b[1], b[2], b[3], b[4]],
        prefix: b[5],
        gw: [b[6], b[7], b[8], b[9]],
        dns: [b[10], b[11], b[12], b[13]],
    };
    NetStatus::Bound { lease, port: bound }
}
