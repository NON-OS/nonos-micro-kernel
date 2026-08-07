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

use crate::types::PkgErr;

const TAGS: [u8; 2] = [1, 2];

pub fn check_trailer(bytes: &[u8], trailer_off: usize) -> Result<(), PkgErr> {
    let t = bytes.get(trailer_off..).ok_or(PkgErr::BadTrailer)?;
    if t.first() != Some(&(TAGS.len() as u8)) {
        return Err(PkgErr::BadTrailer);
    }
    let mut p = 1usize;
    for tag in TAGS {
        if t.len() < p + 3 || t[p] != tag {
            return Err(PkgErr::BadTrailer);
        }
        let len = u16::from_be_bytes([t[p + 1], t[p + 2]]) as usize;
        p += 3;
        if t.len() - p < len {
            return Err(PkgErr::BadTrailer);
        }
        p += len;
    }
    if p != t.len() {
        return Err(PkgErr::BadTrailer);
    }
    Ok(())
}
