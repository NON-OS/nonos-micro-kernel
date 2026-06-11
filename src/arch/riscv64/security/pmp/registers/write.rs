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

use super::super::entry::PmpEntry;
use super::super::error::PmpResult;
use super::addr::write_addr;
use super::cfg::write_cfg;
use super::validate::check_index;

pub fn write_entry(index: usize, entry: &PmpEntry) -> PmpResult<()> {
    check_index(index)?;
    write_addr(index, entry.addr)?;
    write_cfg(index, entry.config.to_cfg_byte())?;
    Ok(())
}

pub fn clear_entry(index: usize) -> PmpResult<()> {
    check_index(index)?;
    write_cfg(index, 0)?;
    write_addr(index, 0)?;
    Ok(())
}
