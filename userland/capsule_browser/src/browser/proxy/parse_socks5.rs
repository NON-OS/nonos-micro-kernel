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

use crate::browser::state::ProxyConfig;

pub fn parse_socks5(input: &str) -> Option<ProxyConfig> {
    let rest = input.strip_prefix("socks5://")?;
    let (host, port_s) = rest.rsplit_once(':')?;
    let port = port_s.parse().ok()?;
    if host.is_empty()
        || host.contains('/')
        || host.contains('@')
        || host.bytes().any(|b| b.is_ascii_whitespace())
        || port == 0
    {
        return None;
    }
    Some(ProxyConfig { host: String::from(host), port })
}
