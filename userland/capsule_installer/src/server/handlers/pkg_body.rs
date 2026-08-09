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

pub(super) fn parse_path(body: &[u8]) -> Option<&[u8]> {
    if body.len() < 2 {
        return None;
    }
    let n = u16::from_le_bytes([body[0], body[1]]) as usize;
    if n == 0 || n > 255 || body.len() < 2 + n {
        return None;
    }
    Some(&body[2..2 + n])
}

pub(super) fn parse_commit(body: &[u8]) -> Option<(&[u8; 32], &[u8])> {
    if body.len() < 34 {
        return None;
    }
    let digest: &[u8; 32] = body[0..32].try_into().ok()?;
    Some((digest, parse_path(&body[32..])?))
}

pub(super) fn parse_name(body: &[u8]) -> Option<&[u8]> {
    if body.len() < 2 {
        return None;
    }
    let n = u16::from_le_bytes([body[0], body[1]]) as usize;
    if n == 0 || n > 64 || body.len() < 2 + n {
        return None;
    }
    Some(&body[2..2 + n])
}
