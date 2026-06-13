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
use core::str::FromStr;

use super::parse::invalid;
use crate::io::Error;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ipv4Addr {
    octets: [u8; 4],
}

impl Ipv4Addr {
    pub const LOCALHOST: Ipv4Addr = Ipv4Addr { octets: [127, 0, 0, 1] };
    pub const UNSPECIFIED: Ipv4Addr = Ipv4Addr { octets: [0, 0, 0, 0] };

    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { octets: [a, b, c, d] }
    }

    pub const fn octets(&self) -> [u8; 4] {
        self.octets
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let o = self.octets;
        write!(f, "{}.{}.{}.{}", o[0], o[1], o[2], o[3])
    }
}

impl FromStr for Ipv4Addr {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let mut octets = [0u8; 4];
        let mut parts = s.split('.');
        for slot in octets.iter_mut() {
            let part = parts.next().ok_or_else(invalid)?;
            *slot = part.parse::<u8>().map_err(|_| invalid())?;
        }
        if parts.next().is_some() {
            return Err(invalid());
        }
        Ok(Self { octets })
    }
}
