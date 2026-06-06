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

pub fn u32_at(buf: &[u8], off: usize) -> Result<u32, &'static str> {
    let end = off.checked_add(4).ok_or("u32 missing")?;
    let bytes = buf.get(off..end).ok_or("u32 missing")?;
    Ok(u32::from_le_bytes(bytes.try_into().map_err(|_| "u32 missing")?))
}
