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

use alloc::string::String;

pub struct Url {
    pub secure: bool,
    pub host: String,
    pub port: u16,
    pub path: String,
}

/// Split `[scheme://]host[:port][/path]`.
///
/// The scheme decides both the default port and whether the request is
/// encrypted, so the two can never disagree. An explicit port does not
/// downgrade `https`: the caller asked for TLS and gets TLS on whatever
/// port they named.
pub fn parse_url(raw: &[u8]) -> Option<Url> {
    let (secure, rest) = split_scheme(raw);
    let (hostport, path) = match rest.iter().position(|&b| b == b'/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, b"/".as_ref()),
    };
    let (host, port) = split_port(hostport, if secure { 443 } else { 80 });
    if host.is_empty() || host.len() > 253 {
        return None;
    }
    Some(Url {
        secure,
        host: String::from_utf8(host.to_vec()).ok()?,
        port,
        path: String::from_utf8(path.to_vec()).ok()?,
    })
}

/// Bare hosts default to TLS. Typing a name should not be the quiet way to
/// get a cleartext request.
fn split_scheme(raw: &[u8]) -> (bool, &[u8]) {
    if raw.starts_with(b"https://") {
        return (true, &raw[8..]);
    }
    if raw.starts_with(b"http://") {
        return (false, &raw[7..]);
    }
    (true, raw)
}

fn split_port(hostport: &[u8], default: u16) -> (&[u8], u16) {
    let Some(i) = hostport.iter().position(|&b| b == b':') else {
        return (hostport, default);
    };
    let mut p: u32 = 0;
    for &b in &hostport[i + 1..] {
        if b.is_ascii_digit() {
            p = p.saturating_mul(10).saturating_add((b - b'0') as u32);
        }
    }
    let port: u16 = if p == 0 || p > u16::MAX as u32 { default } else { p as u16 };
    (&hostport[..i], port)
}
