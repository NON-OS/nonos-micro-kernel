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
use nonos_inflate::gunzip;
use nonos_libc::{mk_time_millis, mk_yield};

use super::conn::{connect, Conn, Rx, Transport};
use super::framing;
use super::http;

pub const MAX_FETCH: usize = 64 * 1024 * 1024;
const CLOSE_WAIT: u8 = 4;
const EMPTY_BUDGET: u32 = 64;
const RETRIES: u32 = 3;
const BACKOFF_MS: i64 = 250;
const MAX_HOPS: u32 = 5;

pub enum Fetched {
    Body(Vec<u8>),
    Redirect(Vec<u8>),
}

fn is_redirect(status: u16) -> bool {
    matches!(status, 301 | 302 | 303 | 307 | 308)
}

pub fn get(
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, &'static str> {
    let mut last = "connect failed";
    for i in 0..RETRIES {
        match attempt(ip, port, host, path, extra) {
            Ok(body) => return Ok(body),
            Err((false, e)) => return Err(e),
            Err((true, e)) => {
                last = e;
                if i + 1 < RETRIES {
                    backoff(BACKOFF_MS);
                }
            }
        }
    }
    Err(last)
}

fn fetch_body(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, &'static str> {
    for _ in 0..2 {
        if conn.is_none() {
            *conn = connect(ip, port);
        }
        let c = match conn {
            Some(c) => c,
            None => break,
        };
        match get_on(&*c, host, path, extra) {
            Ok(f) => return Ok(f),
            Err((false, e)) => return Err(e),
            Err((true, _)) => {
                if let Some(c) = conn.take() {
                    c.close();
                }
            }
        }
    }
    get(ip, port, host, path, extra)
}

pub fn get_reuse(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Vec<u8>, &'static str> {
    let mut cur = path.to_vec();
    for _ in 0..MAX_HOPS {
        match fetch_body(conn, ip, port, host, &cur, extra)? {
            Fetched::Body(b) => return Ok(b),
            Fetched::Redirect(loc) => {
                if let Some(c) = conn.take() {
                    c.close();
                }
                cur = super::redirect::resolve(host, &cur, &loc)?;
            }
        }
    }
    Err("too many redirects")
}

pub fn head_reuse(
    conn: &mut Option<Conn>,
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Option<usize> {
    for _ in 0..2 {
        if conn.is_none() {
            *conn = connect(ip, port);
        }
        let c = match conn {
            Some(c) => c,
            None => return None,
        };
        if let Some(n) = head_len(&*c, host, path, extra) {
            return Some(n);
        }
        if let Some(c) = conn.take() {
            c.close();
        }
    }
    None
}

fn head_len<T: Transport>(conn: &T, host: &[u8], path: &[u8], extra: &[u8]) -> Option<usize> {
    if !conn.send(&http::build_head(host, path, extra)) {
        return None;
    }
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        if let Some(head) = http::parse_head(&raw) {
            if head.status != 200 {
                return None;
            }
            return head.content_length;
        }
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                if conn.state() >= CLOSE_WAIT {
                    return None;
                }
                empties += 1;
                if empties > EMPTY_BUDGET {
                    return None;
                }
            }
            Rx::Gone => return None,
        }
        if raw.len() > MAX_FETCH {
            return None;
        }
    }
}

fn attempt(
    ip: [u8; 4],
    port: u16,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, (bool, &'static str)> {
    let conn = connect(ip, port).ok_or((true, "connect failed"))?;
    if !conn.send(&http::build_get(host, path, extra)) {
        conn.close();
        return Err((true, "send failed"));
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
                    return Err((true, "recv stalled"));
                }
            }
            Rx::Gone => break,
        }
        if raw.len() > MAX_FETCH {
            conn.close();
            return Err((false, "response too large"));
        }
    }
    conn.close();
    finish(&raw).map_err(|e| (false, e))
}

fn backoff(ms: i64) {
    let start = mk_time_millis();
    while mk_time_millis().wrapping_sub(start) < ms {
        mk_yield();
    }
}

fn finish(raw: &[u8]) -> Result<Fetched, &'static str> {
    let head = http::parse_head(raw).ok_or("bad http response")?;
    if head.status != 200 {
        return match head.location {
            Some(loc) if is_redirect(head.status) => Ok(Fetched::Redirect(loc)),
            _ => Err("http status not 200"),
        };
    }
    if head.chunked {
        return Err("chunked transfer unsupported");
    }
    let body = match head.content_length {
        Some(len) => raw.get(head.body_off..head.body_off + len).ok_or("truncated body")?,
        None => &raw[head.body_off..],
    };
    decode(body, head.gzip).map(Fetched::Body).map_err(|(_, e)| e)
}

fn decode(body: &[u8], gzip: bool) -> Result<Vec<u8>, (bool, &'static str)> {
    if gzip {
        let out = gunzip(body).ok_or((false, "gunzip failed"))?;
        if out.len() > MAX_FETCH {
            return Err((false, "response too large"));
        }
        Ok(out)
    } else {
        Ok(body.to_vec())
    }
}

pub fn get_on<T: Transport>(
    conn: &T,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Result<Fetched, (bool, &'static str)> {
    if !conn.send(&http::build_get_ka(host, path, extra)) {
        return Err((true, "send failed"));
    }
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        if let Some(f) = framing::frame(&raw)? {
            if f.status == 200 {
                return decode(&raw[f.body], f.gzip).map(Fetched::Body);
            }
            return match f.location {
                Some(loc) if is_redirect(f.status) => Ok(Fetched::Redirect(loc)),
                _ => Err((false, "http status not 200")),
            };
        }
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                if conn.state() >= CLOSE_WAIT {
                    return Err((true, "truncated body"));
                }
                empties += 1;
                if empties > EMPTY_BUDGET {
                    return Err((true, "recv stalled"));
                }
            }
            Rx::Gone => return Err((true, "connection closed")),
        }
        if raw.len() > MAX_FETCH {
            return Err((false, "response too large"));
        }
    }
}
