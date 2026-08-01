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

use nonos_libc::mk_yield;

use super::super::pull::conn::{connect, Conn, Rx};
use super::super::pull::http;

const EMPTY_BUDGET: u32 = 64;
const MAX_RESP: usize = 64 * 1024;

pub fn put(
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    body: &[u8],
    extra: &[u8],
) -> Result<(), &'static str> {
    let conn = connect(ip, port).ok_or("push: connect failed")?;
    let sent = conn.send(&http::build_put_head(host, path, body.len(), extra))
        && (body.is_empty() || conn.send(body));
    if !sent {
        conn.close();
        return Err("push: send failed");
    }
    let raw = read_response(&conn);
    conn.close();
    check_status(&raw)
}

fn read_response(conn: &Conn) -> Vec<u8> {
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                empties += 1;
                if empties > EMPTY_BUDGET {
                    break;
                }
                mk_yield();
            }
            Rx::Gone => break,
        }
        if raw.len() > MAX_RESP {
            break;
        }
    }
    raw
}

fn check_status(raw: &[u8]) -> Result<(), &'static str> {
    let head = http::parse_head(raw).ok_or("push: bad response")?;
    match head.status {
        200 | 201 | 204 => Ok(()),
        _ => Err("push: server rejected upload"),
    }
}
