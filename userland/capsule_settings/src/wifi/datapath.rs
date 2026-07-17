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

//! Read the driver's data-path frame counts from the status op. These split a
//! stuck DHCP: no TX means net_core handed nothing down, RX frames without parsed
//! frames means the AP replies but the frames never decrypt.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const DRIVER_SERVICE: &[u8] = b"driver.rtl8821ce0";
const WIFI_MAGIC: u32 = 0x5749_4649;
const OP_STATUS: u16 = 4;
const WIFI_HDR: usize = 10;
const STATUS_TIMEOUT_MS: u64 = 500;

/// The driver's TX and RX frame counts since bring-up, plus the number of
/// net_core link-protocol requests it has answered (zero means the stack never
/// reached the radio).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct DataPath {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_ring: u32,
    pub rx_eth: u32,
    pub netif_reqs: u32,
    pub rx_err: u32,
}

/// Query the driver for its data-path counts. `None` when the driver service is
/// absent or does not answer with the counters.
pub fn driver_datapath() -> Option<DataPath> {
    let mut port: u32 = 0;
    let rc = mk_service_lookup(
        DRIVER_SERVICE.as_ptr(),
        DRIVER_SERVICE.len(),
        &mut port as *mut u32,
        ptr::null_mut(),
    );
    if rc != 0 || port == 0 {
        return None;
    }
    let mut req = [0u8; WIFI_HDR];
    req[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&OP_STATUS.to_le_bytes());
    let mut resp = [0u8; WIFI_HDR + 25];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        STATUS_TIMEOUT_MS,
    );
    if n < (WIFI_HDR + 25) as i64 {
        return None;
    }
    let b = &resp[WIFI_HDR + 1..];
    Some(DataPath {
        tx_ok: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        tx_drop: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
        rx_ring: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        rx_eth: u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        netif_reqs: u32::from_le_bytes([b[16], b[17], b[18], b[19]]),
        rx_err: u32::from_le_bytes([b[20], b[21], b[22], b[23]]),
    })
}
