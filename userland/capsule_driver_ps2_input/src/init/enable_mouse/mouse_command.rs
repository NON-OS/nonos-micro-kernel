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
use super::command::command;
use super::read_data::read_data;
use super::write_data::write_data;
use crate::constants::{CTL_WRITE_AUX, MOUSE_ACK};

pub(super) fn mouse_command(grant_id: u64, cmd: u8) -> Result<(), &'static str> {
    command(grant_id, CTL_WRITE_AUX)?;
    write_data(grant_id, cmd)?;
    if read_data(grant_id)? == MOUSE_ACK {
        Ok(())
    } else {
        Err("mouse command not acknowledged")
    }
}
