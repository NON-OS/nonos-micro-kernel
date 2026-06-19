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

use super::lookup::resolve_dns;
use super::parse::parse_ipv4;

pub enum Resolved {
    Ip([u8; 4]),
    NoService,
    Timeout,
    ServFail,
    Unknown,
}

pub fn resolve(host: &[u8]) -> Resolved {
    if let Some(ip) = parse_ipv4(host) {
        return Resolved::Ip(ip);
    }
    resolve_dns(host)
}
