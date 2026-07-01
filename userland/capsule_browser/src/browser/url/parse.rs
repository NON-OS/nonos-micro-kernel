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

use super::host_port::host_port;
use super::normalize_path::normalize_path;
use super::scheme_rest::scheme_rest;
use super::split_path::split_path;
use super::types::Url;

pub fn parse(input: &str) -> Option<Url> {
    let s = input.trim();
    let (scheme, rest) = scheme_rest(s)?;
    if rest.is_empty() {
        return None;
    }
    let (authority, path) = split_path(rest);
    let (host, port) = host_port(authority, scheme)?;
    Some(Url { scheme, host, port, path: normalize_path(path) })
}
