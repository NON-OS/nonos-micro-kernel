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

const BODY_LEN: usize = 17;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    Send,
    BadResponse,
    Refused(u16),
}

pub fn read_ipv4(ip_port: u32) -> Result<[u8; 4], ConfigError> {
    let mut req = [0u8; IP_HDR_LEN];
    write_request(&mut req, OP_GET_CONFIG, seq::next(), 0);
    let mut resp = [0u8; IP_HDR_LEN + BODY_LEN];
    let n = mk_ipc_call(ip_port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(ConfigError::Send);
    }
    let (op, errno, _, plen) = parse_response(&resp).ok_or(ConfigError::BadResponse)?;
    if op != OP_GET_CONFIG || plen as usize != BODY_LEN {
        return Err(ConfigError::BadResponse);
    }
    if errno != 0 {
        return Err(ConfigError::Refused(errno));
    }
    let mut ip = [0u8; 4];
    ip.copy_from_slice(&resp[IP_HDR_LEN + 6..IP_HDR_LEN + 10]);
    Ok(ip)
}
