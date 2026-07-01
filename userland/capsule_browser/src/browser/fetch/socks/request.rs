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

use crate::browser::net;
use crate::browser::url::Url;

pub fn request(url: &Url) -> Option<Vec<u8>> {
    if let Some(ip) = net::parse_ipv4(&url.host) {
        let mut out = Vec::with_capacity(10);
        out.extend_from_slice(&[0x05, 0x01, 0x00, 0x01]);
        out.extend_from_slice(&ip);
        out.extend_from_slice(&url.port.to_be_bytes());
        return Some(out);
    }
    if url.host.is_empty() || url.host.len() > 255 {
        return None;
    }
    let mut out = Vec::with_capacity(url.host.len() + 7);
    out.extend_from_slice(&[0x05, 0x01, 0x00, 0x03, url.host.len() as u8]);
    out.extend_from_slice(url.host.as_bytes());
    out.extend_from_slice(&url.port.to_be_bytes());
    Some(out)
}
