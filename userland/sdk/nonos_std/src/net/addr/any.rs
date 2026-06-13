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

use core::fmt;

use super::v4::SocketAddrV4;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SocketAddr {
    V4(SocketAddrV4),
}

impl SocketAddr {
    pub const fn port(&self) -> u16 {
        match self {
            SocketAddr::V4(a) => a.port(),
        }
    }

    pub(crate) fn v4_parts(&self) -> ([u8; 4], u16) {
        match self {
            SocketAddr::V4(a) => (a.ip().octets(), a.port()),
        }
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocketAddr::V4(a) => a.fmt(f),
        }
    }
}
