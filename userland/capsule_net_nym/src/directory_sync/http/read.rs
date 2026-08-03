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
use nonos_libc::{mk_uptime_ms, mk_yield};

use crate::tcp_client;

const RESPONSE_CAP: usize = 16 * 1024;
/// A directory is fetched across the internet, so the wait is measured in
/// round trips rather than in attempts. Counting reads gave up in
/// microseconds.
const RESPONSE_DEADLINE_MS: i64 = 8_000;

pub fn response(tcp_port: u32, stream: u32) -> Result<Vec<u8>, u16> {
    let mut resp = Vec::with_capacity(RESPONSE_CAP);
    let mut chunk = [0u8; 1024];
    let deadline = mk_uptime_ms().saturating_add(RESPONSE_DEADLINE_MS);
    loop {
        let n = tcp_client::recv(tcp_port, stream, &mut chunk)?;
        if n == 0 {
            if header_done(&resp) {
                return Ok(resp);
            }
            if mk_uptime_ms() >= deadline {
                return Err(20);
            }
            mk_yield();
            continue;
        }
        if resp.len() + n > RESPONSE_CAP {
            return Err(20);
        }
        resp.extend_from_slice(&chunk[..n]);
        if complete(&resp) {
            return Ok(resp);
        }
    }
}

fn complete(resp: &[u8]) -> bool {
    let Some(body) = header_end(resp) else {
        return false;
    };
    match content_length(&resp[..body]) {
        Some(n) => resp.len() >= body + n,
        None => false,
    }
}

fn header_done(resp: &[u8]) -> bool {
    header_end(resp).is_some()
}

fn header_end(resp: &[u8]) -> Option<usize> {
    resp.windows(4).position(|w| w == b"\r\n\r\n").map(|i| i + 4)
}

fn content_length(headers: &[u8]) -> Option<usize> {
    super::parse::content_length(headers)
}
