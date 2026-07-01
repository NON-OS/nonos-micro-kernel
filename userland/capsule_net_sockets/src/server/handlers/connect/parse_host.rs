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

use crate::server::handlers::io::{u16_at, u32_at};

pub fn parse(body: &[u8]) -> Option<(u32, u16, &[u8])> {
    let handle = u32_at(body, 0).ok()?;
    let port = u16_at(body, 4).ok()?;
    let len = u16_at(body, 6).ok()? as usize;
    if len == 0 || len > 253 || 8 + len > body.len() {
        return None;
    }
    Some((handle, port, &body[8..8 + len]))
}
