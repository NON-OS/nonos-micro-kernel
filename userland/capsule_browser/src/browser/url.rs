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

pub fn join(base: &Url, location: &str) -> String {
    let loc = location.trim();
    if loc.starts_with("http://") || loc.starts_with("https://") {
        return loc.to_string();
    }
    let scheme = if base.scheme == Scheme::Https { "https" } else { "http" };
    if let Some(rest) = loc.strip_prefix("//") {
        return alloc::format!("{}://{}", scheme, rest);
    }
    if loc.starts_with('/') {
        return alloc::format!("{}://{}{}", scheme, base.host, loc);
    }
    let dir = match base.path.rfind('/') {
        Some(i) => &base.path[..=i],
        None => "/",
    };
    alloc::format!("{}://{}{}{}", scheme, base.host, dir, loc)
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
