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

//! Ask the RTL8821CE driver to scan. The panel looks the driver service up by
//! name and sends it the one-shot scan control request; the driver hops the
//! channels and replies with the networks it heard, encoded exactly as
//! `wire::parse_scan` reads them. The call blocks for the length of a scan, so
//! the timeout is generous. The three outcomes the panel distinguishes are: the
//! driver is not present, it did not answer in time, or it answered (with zero or
//! more networks) so the list is authoritative.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::network::ScanNetwork;
use super::wire::parse_scan;

/// The driver's registered service name.
const DRIVER_SERVICE: &[u8] = b"driver.rtl8821ce0";
/// The driver's control-family tag and opcodes, and its 10-byte header.
const WIFI_MAGIC: u32 = 0x5749_4649;
const OP_CONNECT: u16 = 1;
const OP_SCAN: u16 = 3;
const OP_STATUS: u16 = 4;
const WIFI_HDR: usize = 10;
/// A join hunts for the beacon then runs the handshake, so allow well past both.
const CONNECT_TIMEOUT_MS: u64 = 20_000;
/// A status reply the driver answers quickly; it does not run a scan.
const STATUS_TIMEOUT_MS: u64 = 500;
/// A scan hops thirteen channels, so wait well past its worst case.
const SCAN_TIMEOUT_MS: u64 = 15_000;
/// Enough for the count byte and the maximum network list.
const RESP_MAX: usize = 1024;

/// What a scan attempt resolved to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScanOutcome {
    /// No driver service is registered, so there is no radio to scan.
    NoService,
    /// The driver did not reply within the scan timeout.
    NoResponse,
    /// The driver answered; the network list (possibly empty) is complete.
    Scanned,
}

/// The driver's receive-pipeline counters, carried in the scan reply so the panel
/// can show where reception stops on a machine whose boot console the desktop
/// hides: background scan passes taken, raw frames the ring delivered, and beacons
/// parsed from them.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ScanStats {
    pub steps: u32,
    pub raw: u32,
    pub beacons: u32,
}

/// How far the driver's radio bring-up got, mirroring the driver's stage codes.
/// A `NotReady` stage is why a scan would find nothing, so the panel shows it
/// rather than an empty list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DriverStage {
    Ready,
    NotClaimed,
    PowerFailed,
    DeadMmio,
    FirmwareFailed,
    NoDma,
    EfuseFailed,
    /// The radio came up but no station address could be drawn, so it was left
    /// dark rather than transmitting under a predictable one.
    NoStationAddress,
    Unknown,
}

impl DriverStage {
    fn from_code(code: u8) -> Self {
        match code {
            0 => DriverStage::Ready,
            1 => DriverStage::NotClaimed,
            2 => DriverStage::PowerFailed,
            3 => DriverStage::DeadMmio,
            4 => DriverStage::FirmwareFailed,
            5 => DriverStage::NoDma,
            6 => DriverStage::EfuseFailed,
            7 => DriverStage::NoStationAddress,
            _ => DriverStage::Unknown,
        }
    }
}

/// Ask the driver how far its radio bring-up got. `None` if there is no driver
/// service or it did not answer.
pub fn driver_stage() -> Option<DriverStage> {
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
    let mut resp = [0u8; WIFI_HDR + 1];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        STATUS_TIMEOUT_MS,
    );
    if n < (WIFI_HDR + 1) as i64 {
        return None;
    }
    Some(DriverStage::from_code(resp[WIFI_HDR]))
}

/// The counters sit between the header and the network list: three u32s.
const STATS_LEN: usize = 12;

/// A connection attempt's result: the driver status code, and the handshake
/// progress (frames sent and received, and the state the machine reached) for
/// diagnosing a failed join.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ConnectResult {
    pub code: i32,
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub probe: u32,
    pub deauth: u32,
    pub to_us: u32,
    pub state: u8,
}

/// Ask the driver to join `ssid` with `pass`. Blocks for the whole join: the
/// driver hunts the beacon, runs open authentication, association and the WPA2
/// four-way handshake, then installs the keys. Returns the driver status code
/// (0 = associated, negative = a specific failure) with the handshake progress,
/// or a synthetic negative when the driver could not be reached.
pub fn connect_network(ssid: &[u8], pass: &[u8]) -> ConnectResult {
    let fail = |code| ConnectResult { code, ..Default::default() };
    let mut port: u32 = 0;
    let rc = mk_service_lookup(
        DRIVER_SERVICE.as_ptr(),
        DRIVER_SERVICE.len(),
        &mut port as *mut u32,
        ptr::null_mut(),
    );
    if rc != 0 || port == 0 {
        return fail(-100);
    }

    // [header][ssid_len][ssid][pass_len][pass].
    let mut req = [0u8; WIFI_HDR + 1 + 32 + 1 + 64];
    req[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&OP_CONNECT.to_le_bytes());
    let mut o = WIFI_HDR;
    let sl = ssid.len().min(32);
    req[o] = sl as u8;
    o += 1;
    req[o..o + sl].copy_from_slice(&ssid[..sl]);
    o += sl;
    let pl = pass.len().min(64);
    req[o] = pl as u8;
    o += 1;
    req[o..o + pl].copy_from_slice(&pass[..pl]);
    o += pl;

    let mut resp = [0u8; 48];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        o,
        resp.as_mut_ptr(),
        resp.len(),
        CONNECT_TIMEOUT_MS,
    );
    if n < 14 {
        return fail(-101);
    }
    let code = i32::from_le_bytes([resp[10], resp[11], resp[12], resp[13]]);
    // The extended reply carries the handshake progress after the status code.
    if n < 43 {
        return ConnectResult { code, ..Default::default() };
    }
    ConnectResult {
        code,
        sent: u32::from_le_bytes([resp[14], resp[15], resp[16], resp[17]]),
        recv: u32::from_le_bytes([resp[18], resp[19], resp[20], resp[21]]),
        data: u32::from_le_bytes([resp[22], resp[23], resp[24], resp[25]]),
        eapol: u32::from_le_bytes([resp[26], resp[27], resp[28], resp[29]]),
        probe: u32::from_le_bytes([resp[30], resp[31], resp[32], resp[33]]),
        deauth: u32::from_le_bytes([resp[34], resp[35], resp[36], resp[37]]),
        to_us: u32::from_le_bytes([resp[38], resp[39], resp[40], resp[41]]),
        state: resp[42],
    }
}

/// Run one scan, filling `out` with up to its length networks. Returns how many
/// were written, how the attempt resolved, and the driver's receive-pipeline
/// counters (zeroed when the driver did not answer with them).
pub fn scan_networks(out: &mut [ScanNetwork]) -> (usize, ScanOutcome, ScanStats) {
    let mut port: u32 = 0;
    let rc = mk_service_lookup(
        DRIVER_SERVICE.as_ptr(),
        DRIVER_SERVICE.len(),
        &mut port as *mut u32,
        ptr::null_mut(),
    );
    if rc != 0 || port == 0 {
        return (0, ScanOutcome::NoService, ScanStats::default());
    }

    let mut req = [0u8; WIFI_HDR];
    req[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&OP_SCAN.to_le_bytes());
    // Request id and any body stay zero; the scan takes no arguments.

    let mut resp = [0u8; RESP_MAX];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        SCAN_TIMEOUT_MS,
    );
    if n < (WIFI_HDR + STATS_LEN) as i64 {
        return (0, ScanOutcome::NoResponse, ScanStats::default());
    }

    let n = n as usize;
    let stats = ScanStats {
        steps: u32::from_le_bytes([
            resp[WIFI_HDR],
            resp[WIFI_HDR + 1],
            resp[WIFI_HDR + 2],
            resp[WIFI_HDR + 3],
        ]),
        raw: u32::from_le_bytes([
            resp[WIFI_HDR + 4],
            resp[WIFI_HDR + 5],
            resp[WIFI_HDR + 6],
            resp[WIFI_HDR + 7],
        ]),
        beacons: u32::from_le_bytes([
            resp[WIFI_HDR + 8],
            resp[WIFI_HDR + 9],
            resp[WIFI_HDR + 10],
            resp[WIFI_HDR + 11],
        ]),
    };
    let mut count = 0;
    parse_scan(&resp[WIFI_HDR + STATS_LEN..n], |net| {
        if count < out.len() {
            out[count] = net;
            count += 1;
        }
    });
    (count, ScanOutcome::Scanned, stats)
}
