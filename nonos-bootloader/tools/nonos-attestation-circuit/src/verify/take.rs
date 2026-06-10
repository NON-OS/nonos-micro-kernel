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

pub(super) fn take<'a>(data: &'a [u8], pos: &mut usize, len: usize) -> Result<&'a [u8], String> {
    let end = pos.checked_add(len).ok_or("offset overflow")?;
    if end > data.len() {
        return Err("truncated capsule ZK trailer".into());
    }
    let out = &data[*pos..end];
    *pos = end;
    Ok(out)
}
