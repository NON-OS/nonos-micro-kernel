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

use alloc::string::{String, ToString};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Http,
    Https,
}

#[derive(Clone)]
pub struct Url {
    pub scheme: Scheme,
    pub host: String,
    pub port: u16,
    pub path: String,
}

pub fn parse(input: &str) -> Option<Url> {
    let s = input.trim();
    let (scheme, rest) = if let Some(r) = s.strip_prefix("https://") {
        (Scheme::Https, r)
    } else if let Some(r) = s.strip_prefix("http://") {
        (Scheme::Http, r)
    } else {
        (Scheme::Https, s)
    };
    if rest.is_empty() {
        return None;
    }
    let (authority, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let (host, port) = split_host_port(authority, scheme)?;
    Some(Url { scheme, host, port, path: path.to_string() })
}

fn split_host_port(authority: &str, scheme: Scheme) -> Option<(String, u16)> {
    let default = if scheme == Scheme::Https { 443 } else { 80 };
    match authority.rsplit_once(':') {
        Some((h, p)) if !h.is_empty() => Some((h.to_string(), p.parse().ok()?)),
        _ => {
            if authority.is_empty() {
                None
            } else {
                Some((authority.to_string(), default))
            }
        }
    }
}
