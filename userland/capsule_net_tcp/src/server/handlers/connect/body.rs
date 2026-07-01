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

use crate::protocol::{E_BAD_ADDR, E_BAD_LEN};
use crate::server::handlers::io::{ip4_at, u16_at};

pub fn parse(body: &[u8]) -> Result<([u8; 4], u16), u16> {
    if body.len() < 6 {
        return Err(E_BAD_LEN);
    }
    let ip = ip4_at(body, 0)?;
    let port = u16_at(body, 4)?;
    if ip == [0, 0, 0, 0] || port == 0 {
        return Err(E_BAD_ADDR);
    }
    Ok((ip, port))
}
