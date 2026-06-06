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
use super::cmd::cmd;
use super::data::data;
use super::read_byte::read_byte;
use super::reset::reset;
use crate::constants::{
    CONFIG_IRQ1, CONFIG_KBD_DISABLE, CTL_ENABLE_KBD, CTL_READ_CONFIG, CTL_WRITE_CONFIG,
};
use crate::init::enable_scanning;

pub fn enable_keyboard(grant_id: u64) -> Result<(), &'static str> {
    cmd(grant_id, CTL_ENABLE_KBD)?;
    cmd(grant_id, CTL_READ_CONFIG)?;
    let cfg = read_byte(grant_id)?;
    if cfg == 0xFF {
        return Err("kbd config invalid");
    }
    cmd(grant_id, CTL_WRITE_CONFIG)?;
    data(grant_id, (cfg | CONFIG_IRQ1) & !CONFIG_KBD_DISABLE)?;
    reset(grant_id);
    enable_scanning(grant_id)
}
