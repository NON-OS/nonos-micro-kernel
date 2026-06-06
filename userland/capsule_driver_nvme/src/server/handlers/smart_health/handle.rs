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

use nonos_libc::mk_ipc_send;

use super::put128::put128;
use super::put16::put16;
use super::put32::put32;
use super::put_i16::put_i16;
use crate::protocol::{
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN,
    SMART_HEALTH_PAYLOAD_LEN,
};
use crate::setup::Driver;

pub fn handle(driver: &Driver, req: &Request, tx: &mut [u8]) {
    let h = driver.health;
    let payload_len = (4 + SMART_HEALTH_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut o = RESP_HDR_LEN + 4;
    tx[o] = h.critical_warning;
    o += 1;
    put16(tx, &mut o, h.temperature_kelvin);
    put_i16(tx, &mut o, h.temperature_celsius());
    tx[o] = h.available_spare;
    tx[o + 1] = h.available_spare_threshold;
    tx[o + 2] = h.percentage_used;
    tx[o + 3] = h.endurance_group_warning;
    o += 4;
    for v in [
        h.data_units_read,
        h.data_units_written,
        h.host_read_commands,
        h.host_write_commands,
        h.controller_busy_time,
        h.power_cycles,
        h.power_on_hours,
        h.unsafe_shutdowns,
        h.media_errors,
        h.error_log_entries,
    ] {
        put128(tx, &mut o, v);
    }
    put32(tx, &mut o, h.warning_temp_time);
    put32(tx, &mut o, h.critical_temp_time);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
}
