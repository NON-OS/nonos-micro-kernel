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
use super::pio_write::pio_write;
use super::wait_clear::wait_clear;
use crate::constants::STATUS_OFFSET;

pub(super) fn cmd(grant_id: u64, value: u8) -> Result<(), &'static str> {
    wait_clear(grant_id)?;
    pio_write(grant_id, STATUS_OFFSET, value)
}
