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

// A real HTTP/1.0 client over net.sockets: resolve the host through the DNS
// path (OP_CONNECT_HOST), open a stream, send a GET, and stream the reply back
// to the terminal. Works against the live internet on real hardware, not a lab
// address: `http example.com`, `http 1.1.1.1`, `http http://host:8080/path`.

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use crate::term::state::State;

const SERVICE: &[u8] = b"net.sockets";
const MAGIC: u32 = 0x4E53_4B54;
const HDR: usize = 20;
const OP_SOCKET: u16 = 2;
const OP_SEND: u16 = 7;
const OP_RECV: u16 = 8;
const OP_CLOSE: u16 = 9;
const OP_CONNECT_HOST: u16 = 12;
const FAMILY_IP4: u16 = 4;
const KIND_STREAM: u16 = 1;

const CALL_MS: u64 = 4000;
const CONNECT_MS: u64 = 9000;
const RECV_MS: u64 = 1500;
const RECV_CHUNK: usize = 4096;
// Cap on how much of the response we buffer and print, so a large page cannot
// exhaust the terminal heap or flood the scrollback.
const MAX_BODY: usize = 64 * 1024;

// Frame a net.sockets request, call, and validate the reply envelope (magic
// header + zero errno). Returns the reply length on success.
fn call(port: u32, op: u16, body: &[u8], rx: &mut [u8], timeout: u64) -> Option<usize> {
    let mut tx: Vec<u8> = Vec::with_capacity(HDR + body.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    let n =
        mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len(), timeout);
    if n < HDR as i64 {
        return None;
    }
    if u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return None;
    }
    Some(n as usize)
}

fn close(port: u32, handle: u32) {
    let mut rx = [0u8; HDR];
    let _ = call(port, OP_CLOSE, &handle.to_le_bytes(), &mut rx, CALL_MS);
}

// Split `[scheme://]host[:port][/path]` into its parts, defaulting to port 80
// and path "/". No scheme, no path, and bare IPs all work.
fn parse_url(raw: &[u8]) -> (Vec<u8>, u16, Vec<u8>) {
    let mut s = raw;
    for scheme in [b"http://".as_ref(), b"https://".as_ref()] {
        if s.len() >= scheme.len() && &s[..scheme.len()] == scheme {
            s = &s[scheme.len()..];
            break;
        }
    }
    let (hostport, path) = match s.iter().position(|&b| b == b'/') {
        Some(i) => (&s[..i], &s[i..]),
        None => (s, b"/".as_ref()),
    };
    let (host, port) = match hostport.iter().position(|&b| b == b':') {
        Some(i) => {
            let mut p: u32 = 0;
            for &b in &hostport[i + 1..] {
                if b.is_ascii_digit() {
                    p = p.saturating_mul(10).saturating_add((b - b'0') as u32);
                }
            }
            (&hostport[..i], p as u16)
        }
        None => (hostport, 80u16),
    };
    let port = if port == 0 { 80 } else { port };
    let path = if path.is_empty() { b"/".to_vec() } else { path.to_vec() };
    (host.to_vec(), port, path)
}

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&url) = args.first() else {
        state.scrollback.push_error(
            b"usage: http <url>   e.g. http example.com  |  http 1.1.1.1  |  http host:8080/path",
        );
        return false;
    };
    let (host, port, path) = parse_url(url);
    if host.is_empty() || host.len() > 253 {
        state.scrollback.push_error(b"http: bad host");
        return false;
    }

    let mut sp = 0u32;
    let mut spid = 0u32;
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut sp, &mut spid) < 0 || sp == 0 {
        state.scrollback.push_error(b"http: net.sockets unavailable");
        return false;
    }

    // open a stream socket
    let mut orx = [0u8; 64];
    let mut ob = [0u8; 4];
    ob[0..2].copy_from_slice(&FAMILY_IP4.to_le_bytes());
    ob[2..4].copy_from_slice(&KIND_STREAM.to_le_bytes());
    let handle = match call(sp, OP_SOCKET, &ob, &mut orx, CALL_MS) {
        Some(n) if n >= 24 => u32::from_le_bytes([orx[20], orx[21], orx[22], orx[23]]),
        _ => {
            state.scrollback.push_error(b"http: socket open failed");
            return false;
        }
    };

    // resolve + connect
    let mut cb = Vec::with_capacity(8 + host.len());
    cb.extend_from_slice(&handle.to_le_bytes());
    cb.extend_from_slice(&port.to_le_bytes());
    cb.extend_from_slice(&(host.len() as u16).to_le_bytes());
    cb.extend_from_slice(&host);
    let mut crx = [0u8; 32];
    if call(sp, OP_CONNECT_HOST, &cb, &mut crx, CONNECT_MS).is_none() {
        state.scrollback.push_error(b"http: connect failed (dns or route)");
        close(sp, handle);
        return false;
    }

    // send GET
    let mut req: Vec<u8> = Vec::new();
    req.extend_from_slice(b"GET ");
    req.extend_from_slice(&path);
    req.extend_from_slice(b" HTTP/1.0\r\nHost: ");
    req.extend_from_slice(&host);
    req.extend_from_slice(b"\r\nUser-Agent: nonos-http/1.0\r\nConnection: close\r\n\r\n");
    let mut sb = Vec::with_capacity(4 + req.len());
    sb.extend_from_slice(&handle.to_le_bytes());
    sb.extend_from_slice(&req);
    let mut srx = [0u8; 32];
    if call(sp, OP_SEND, &sb, &mut srx, CALL_MS).is_none() {
        state.scrollback.push_error(b"http: send failed");
        close(sp, handle);
        return false;
    }

    // stream the reply until the peer closes or the cap is hit
    let mut resp: Vec<u8> = Vec::new();
    let mut idle = 0u32;
    while resp.len() < MAX_BODY {
        let mut rrx = vec![0u8; RECV_CHUNK + HDR];
        let n = match call(sp, OP_RECV, &handle.to_le_bytes(), &mut rrx, RECV_MS) {
            Some(n) => n,
            None => break,
        };
        let payload = u32::from_le_bytes([rrx[16], rrx[17], rrx[18], rrx[19]]) as usize;
        let got = core::cmp::min(payload, n.saturating_sub(HDR));
        if got == 0 {
            idle += 1;
            if idle >= 4 {
                break;
            }
            continue;
        }
        idle = 0;
        resp.extend_from_slice(&rrx[HDR..HDR + got]);
    }
    close(sp, handle);

    if resp.is_empty() {
        state.scrollback.push_error(b"http: no response");
        return false;
    }
    // print the reply line by line, stripping the trailing CR
    for line in resp.split(|&b| b == b'\n') {
        let line = match line.split_last() {
            Some((&b'\r', head)) => head,
            _ => line,
        };
        state.scrollback.push_line(line);
    }
    true
}
