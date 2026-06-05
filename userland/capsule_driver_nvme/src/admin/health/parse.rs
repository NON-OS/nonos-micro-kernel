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

use super::health_type::SmartHealth;
use super::le128::le128;
use super::le16::le16;
use super::le32::le32;

impl SmartHealth {
    pub fn parse(data: &[u8]) -> Self {
        Self {
            critical_warning: data[0],
            temperature_kelvin: le16(data, 1),
            available_spare: data[3],
            available_spare_threshold: data[4],
            percentage_used: data[5],
            endurance_group_warning: data[6],
            data_units_read: le128(data, 32),
            data_units_written: le128(data, 48),
            host_read_commands: le128(data, 64),
            host_write_commands: le128(data, 80),
            controller_busy_time: le128(data, 96),
            power_cycles: le128(data, 112),
            power_on_hours: le128(data, 128),
            unsafe_shutdowns: le128(data, 144),
            media_errors: le128(data, 160),
            error_log_entries: le128(data, 176),
            warning_temp_time: le32(data, 192),
            critical_temp_time: le32(data, 196),
        }
    }
}
