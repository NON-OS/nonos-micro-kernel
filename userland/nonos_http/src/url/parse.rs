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
//! Parsing a URL a request can be built from.

extern crate alloc;

use alloc::string::String;

use super::host::{is_host_byte, is_path_byte};
use super::types::Url;

/// Longest url this will look at, so a caller cannot be handed one that costs
/// more to validate than it could ever be worth.
const MAX_URL: usize = 2048;

/// Parse `https://host/path`.
///
/// Only HTTPS. Over plain HTTP anyone on the path can serve the response, and
/// a caller has no way to tell that happened.
pub fn parse(url: &str) -> Option<Url> {
    if url.len() > MAX_URL {
        return None;
    }
    let rest = url.strip_prefix("https://")?;
    let slash = rest.find('/')?;
    let (host, path) = rest.split_at(slash);
    if host.is_empty() || host.len() > 253 {
        return None;
    }
    if !host.bytes().all(is_host_byte) || !path.bytes().all(is_path_byte) {
        return None;
    }
    let path = path.trim_end_matches('/');
    if path.len() < 2 {
        return None;
    }
    Some(Url { host: String::from(host), path: String::from(path) })
}
