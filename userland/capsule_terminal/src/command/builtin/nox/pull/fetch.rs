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

use alloc::vec::Vec;

use super::conn::{connect, Rx};
use super::http;

pub const MAX_FETCH: usize = 64 * 1024 * 1024;
const CLOSE_WAIT: u8 = 4;
const EMPTY_BUDGET: u32 = 64;

pub fn get(ip: [u8; 4], port: u16, host: &[u8], path: &[u8]) -> Result<Vec<u8>, &'static str> {
    let conn = connect(ip, port).ok_or("connect failed")?;
    if !conn.send(&http::build_get(host, path)) {
        conn.close();
        return Err("send failed");
    }
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                if conn.state() >= CLOSE_WAIT {
                    break;
                }
                empties += 1;
                if empties > EMPTY_BUDGET {
                    conn.close();
                    return Err("recv stalled");
                }
            }
            Rx::Gone => break,
        }
        if raw.len() > MAX_FETCH {
            conn.close();
            return Err("response too large");
        }
    }
    conn.close();
    finish(&raw)
}

fn finish(raw: &[u8]) -> Result<Vec<u8>, &'static str> {
    let head = http::parse_head(raw).ok_or("bad http response")?;
    if head.status != 200 {
        return Err("http status not 200");
    }
    if head.chunked {
        return Err("chunked transfer unsupported");
    }
    match head.content_length {
        Some(len) => raw
            .get(head.body_off..head.body_off + len)
            .map(|b| b.to_vec())
            .ok_or("truncated body"),
        None => Ok(raw[head.body_off..].to_vec()),
    }
}
