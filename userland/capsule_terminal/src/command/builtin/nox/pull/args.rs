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

use super::ipv4::parse_ipv4;

pub struct PullArgs {
    pub ip: [u8; 4],
    pub port: u16,
    pub host: Vec<u8>,
    pub path: Vec<u8>,
    pub dest: Vec<u8>,
    pub is_dir: bool,
}

pub fn parse(argv: &[&[u8]]) -> Result<PullArgs, &'static str> {
    if argv.len() != 2 {
        return Err("usage: nox pull <ip:port>/<path> <dest>");
    }
    let target = argv[0];
    let slash = target.iter().position(|&c| c == b'/').ok_or("missing /<path>")?;
    let (hostport, path) = target.split_at(slash);
    let colon = hostport.iter().position(|&c| c == b':').ok_or("missing :port")?;
    let ip = parse_ipv4(&hostport[..colon]).ok_or("bad ipv4")?;
    let port = parse_port(&hostport[colon + 1..])?;
    Ok(PullArgs {
        ip,
        port,
        host: hostport.to_vec(),
        path: path.to_vec(),
        dest: argv[1].to_vec(),
        is_dir: path.last() == Some(&b'/'),
    })
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
