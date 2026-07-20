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

pub struct Target {
    pub host: Vec<u8>,
    pub hostname: Vec<u8>,
    pub port: u16,
    pub path: Vec<u8>,
    pub is_dir: bool,
}

pub fn parse_target(target: &[u8]) -> Result<Target, &'static str> {
    let slash = target.iter().position(|&c| c == b'/').ok_or("missing /<path>")?;
    let (hostport, path) = target.split_at(slash);
    if hostport.is_empty() {
        return Err("missing host");
    }
    let (hostname, port) = match hostport.iter().position(|&c| c == b':') {
        Some(colon) => (&hostport[..colon], parse_port(&hostport[colon + 1..])?),
        None => (hostport, 80u16),
    };
    if hostname.is_empty() {
        return Err("missing host");
    }
    Ok(Target {
        host: hostport.to_vec(),
        hostname: hostname.to_vec(),
        port,
        path: path.to_vec(),
        is_dir: path.last() == Some(&b'/'),
    })
}

pub fn default_dest(t: &Target) -> Vec<u8> {
    let trimmed = if t.is_dir { &t.path[..t.path.len() - 1] } else { &t.path[..] };
    if trimmed.is_empty() {
        let mut d = t.hostname.clone();
        if t.is_dir {
            d.push(b'/');
        }
        return d;
    }
    match trimmed.iter().rposition(|&c| c == b'/') {
        Some(i) => {
            let mut d = t.path[i + 1..].to_vec();
            if d.is_empty() {
                d.push(b'.');
            }
            d
        }
        None => t.path.clone(),
    }
}

fn parse_port(s: &[u8]) -> Result<u16, &'static str> {
    if s.is_empty() {
        return Err("empty port");
    }
    let mut n: u32 = 0;
    for &c in s {
        if !c.is_ascii_digit() {
            return Err("bad port");
        }
        n = n * 10 + (c - b'0') as u32;
        if n > 65535 {
            return Err("port range");
        }
    }
    Ok(n as u16)
}
