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

use super::super::error::DriverNvmeError;
use super::super::protocol::SMART_HEALTH_PAYLOAD_LEN;
use super::read::{i16_at, u128_at, u16_at, u32_at};
use super::smart_health::NvmeSmartHealth;

pub(super) fn decode(body: &[u8]) -> Result<NvmeSmartHealth, DriverNvmeError> {
    if body.len() < SMART_HEALTH_PAYLOAD_LEN {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    Ok(NvmeSmartHealth {
        critical_warning: body[0],
        temperature_kelvin: u16_at(body, 1),
        temperature_celsius: i16_at(body, 3),
        available_spare: body[5],
        available_spare_threshold: body[6],
        percentage_used: body[7],
        endurance_group_warning: body[8],
        data_units_read: u128_at(body, 9),
        data_units_written: u128_at(body, 25),
        host_read_commands: u128_at(body, 41),
        host_write_commands: u128_at(body, 57),
        controller_busy_time: u128_at(body, 73),
        power_cycles: u128_at(body, 89),
        power_on_hours: u128_at(body, 105),
        unsafe_shutdowns: u128_at(body, 121),
        media_errors: u128_at(body, 137),
        error_log_entries: u128_at(body, 153),
        warning_temp_time: u32_at(body, 169),
        critical_temp_time: u32_at(body, 173),
    })
}
