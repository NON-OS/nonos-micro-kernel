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
use super::mouse_command::mouse_command;
use super::read_data::read_data;
use super::write_data::write_data;
use crate::constants::{
    CONFIG_AUX_DISABLE, CONFIG_IRQ1, CONFIG_IRQ12, CTL_ENABLE_AUX, CTL_READ_CONFIG,
    CTL_WRITE_CONFIG, MOUSE_ENABLE_REPORTING, MOUSE_SET_DEFAULTS,
};
use crate::init::flush_output;

pub fn enable_mouse(grant_id: u64) -> Result<(), &'static str> {
    command(grant_id, CTL_ENABLE_AUX)?;
    flush_output(grant_id);
    command(grant_id, CTL_READ_CONFIG)?;
    let cfg = read_data(grant_id)?;
    command(grant_id, CTL_WRITE_CONFIG)?;
    write_data(grant_id, (cfg | CONFIG_IRQ1 | CONFIG_IRQ12) & !CONFIG_AUX_DISABLE)?;
    mouse_command(grant_id, MOUSE_SET_DEFAULTS)?;
    mouse_command(grant_id, MOUSE_ENABLE_REPORTING)?;
    Ok(())
}
