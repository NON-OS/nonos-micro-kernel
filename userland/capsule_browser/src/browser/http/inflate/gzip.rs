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

use alloc::vec::Vec;

use super::inflate_raw::inflate;

pub fn gunzip(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 18 || data[0] != 0x1f || data[1] != 0x8b || data[2] != 8 {
        return None;
    }
    let mut p = 10usize;
    skip_extra(data, &mut p)?;
    skip_name(data, &mut p)?;
    skip_comment(data, &mut p)?;
    if data[3] & 2 != 0 {
        p = p.checked_add(2)?;
    }
    if p >= data.len().saturating_sub(8) {
        return None;
    }
    inflate(&data[p..data.len() - 8])
}

fn skip_extra(data: &[u8], p: &mut usize) -> Option<()> {
    if data[3] & 4 == 0 {
        return Some(());
    }
    let xlen = *data.get(*p)? as usize | ((*data.get(*p + 1)? as usize) << 8);
    *p = p.checked_add(2)?.checked_add(xlen)?;
    (*p < data.len()).then_some(())
}

fn skip_name(data: &[u8], p: &mut usize) -> Option<()> {
    if data[3] & 8 == 0 {
        return Some(());
    }
    while *data.get(*p)? != 0 {
        *p += 1;
    }
    *p += 1;
    Some(())
}

fn skip_comment(data: &[u8], p: &mut usize) -> Option<()> {
    if data[3] & 16 == 0 {
        return Some(());
    }
    while *data.get(*p)? != 0 {
        *p += 1;
    }
    *p += 1;
    Some(())
}
