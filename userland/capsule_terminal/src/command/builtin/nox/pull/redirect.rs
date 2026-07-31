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

pub fn resolve(host: &[u8], cur: &[u8], location: &[u8]) -> Result<Vec<u8>, &'static str> {
    let loc = lstrip(location);
    if loc.is_empty() {
        return Err("redirect: empty location");
    }
    if scheme(loc, b"https://").is_some() {
        return Err("redirect: https unsupported");
    }
    if let Some(rest) = scheme(loc, b"http://") {
        return absolute(host, rest);
    }
    if loc[0] == b'/' {
        return Ok(loc.to_vec());
    }
    Ok(join(cur, loc))
}

fn absolute(host: &[u8], rest: &[u8]) -> Result<Vec<u8>, &'static str> {
    let slash = rest.iter().position(|&c| c == b'/').unwrap_or(rest.len());
    let auth = &rest[..slash];
    let name = &auth[..auth.iter().position(|&c| c == b':').unwrap_or(auth.len())];
    if name != host {
        return Err("redirect: cross-host unsupported");
    }
    Ok(if slash == rest.len() { b"/".to_vec() } else { rest[slash..].to_vec() })
}

fn join(cur: &[u8], rel: &[u8]) -> Vec<u8> {
    let dir = cur.iter().rposition(|&c| c == b'/').map_or(0, |i| i + 1);
    let mut out = Vec::with_capacity(dir + rel.len());
    out.extend_from_slice(&cur[..dir]);
    out.extend_from_slice(rel);
    out
}

fn lstrip(s: &[u8]) -> &[u8] {
    let n = s.iter().take_while(|&&c| c == b' ' || c == b'\t').count();
    &s[n..]
}

fn scheme<'a>(s: &'a [u8], p: &[u8]) -> Option<&'a [u8]> {
    (s.len() >= p.len() && s[..p.len()].eq_ignore_ascii_case(p)).then(|| &s[p.len()..])
}
