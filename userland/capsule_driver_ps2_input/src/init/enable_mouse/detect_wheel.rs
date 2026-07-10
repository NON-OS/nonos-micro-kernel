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

//! The IntelliMouse knock: set sample rate 200, then 100, then 80, and read the
//! device id. A mouse that reports id 3 has switched to the 4-byte report with a
//! scroll wheel. Anything else stays a plain 3-byte mouse.

use super::command::command;
use super::mouse_command::mouse_command;
use super::read_data::read_data;
use super::write_data::write_data;
use crate::constants::{
    CTL_WRITE_AUX, INTELLIMOUSE_ID, MOUSE_ACK, MOUSE_GET_DEVICE_ID, MOUSE_SET_SAMPLE_RATE,
};

pub(super) fn detect_wheel(grant_id: u64) -> Result<bool, &'static str> {
    set_sample_rate(grant_id, 200)?;
    set_sample_rate(grant_id, 100)?;
    set_sample_rate(grant_id, 80)?;
    command(grant_id, CTL_WRITE_AUX)?;
    write_data(grant_id, MOUSE_GET_DEVICE_ID)?;
    if read_data(grant_id)? != MOUSE_ACK {
        return Err("device id not acknowledged");
    }
    Ok(read_data(grant_id)? == INTELLIMOUSE_ID)
}

fn set_sample_rate(grant_id: u64, rate: u8) -> Result<(), &'static str> {
    mouse_command(grant_id, MOUSE_SET_SAMPLE_RATE)?;
    mouse_command(grant_id, rate)
}
