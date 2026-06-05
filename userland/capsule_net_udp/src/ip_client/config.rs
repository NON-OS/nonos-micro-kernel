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

use nonos_libc::mk_ipc_call;

use super::header::{parse_response, write_request};
use super::seq;
use super::wire::{IP_HDR_LEN, OP_GET_CONFIG};

// Response body: 6 MAC + 4 IPv4 + 1 prefix + 4 gateway + 2 MTU.
const BODY_LEN: usize = 17;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    SendFailed,
    BadResponse,
    Refused(u16),
}

// Pull the IP capsule's interface config. UDP needs the source
// IPv4 to seal the pseudo-header checksum on outbound segments.
pub fn read_ipv4(ip_port: u32) -> Result<[u8; 4], ConfigError> {
    let mut req = [0u8; IP_HDR_LEN];
    let rid = seq::next();
    write_request(&mut req, OP_GET_CONFIG, rid, 0);
    let mut resp = [0u8; IP_HDR_LEN + BODY_LEN];
    let n = mk_ipc_call(ip_port as u64, req.as_ptr(), IP_HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(ConfigError::SendFailed);
    }
    let (op, errno, _, plen) = parse_response(&resp).ok_or(ConfigError::BadResponse)?;
    if op != OP_GET_CONFIG {
        return Err(ConfigError::BadResponse);
    }
    if errno != 0 {
        return Err(ConfigError::Refused(errno));
    }
    if (plen as usize) < BODY_LEN {
        return Err(ConfigError::BadResponse);
    }
    let mut ipv4 = [0u8; 4];
    ipv4.copy_from_slice(&resp[IP_HDR_LEN + 6..IP_HDR_LEN + 10]);
    Ok(ipv4)
}
