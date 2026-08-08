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

//! The Wi-Fi tab body: the radio the broker discovered on one line, then the
//! networks the last scan found, one per row with a lock marker when secured and
//! the selection highlight. A status line below always states where the scan
//! stands, so even a scan that found nothing (or a driver that did not answer) is
//! legible rather than blank.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::state::{State, WifiConnect, WifiScan};
use crate::settings::theme::{LABEL_FG, ROW_SELECTED_BG, STATUS_FG_IDLE, VALUE_FG};
use crate::wifi::{DriverStage, ScanOutcome};

use super::layout::{BODY_TOP, LABEL_LEFT, ROW_H};

const HEX: &[u8; 16] = b"0123456789ABCDEF";
/// The column where a secured network's lock marker is drawn.
const SECURED_COL: u32 = 300;

// Render a u16 as four uppercase hex digits.
fn hex4(value: u16, out: &mut [u8; 4]) {
    for (i, byte) in out.iter_mut().rev().enumerate() {
        *byte = HEX[((value >> (i * 4)) & 0xF) as usize];
    }
}

// The driver bring-up stage as a line, or None when the radio is ready (in which
// case the scan result speaks for itself).
fn stage_message(stage: DriverStage) -> Option<&'static [u8]> {
    match stage {
        DriverStage::Ready => None,
        DriverStage::NotClaimed => Some(b"Wi-Fi radio: could not claim the device."),
        DriverStage::PowerFailed => Some(b"Wi-Fi radio: power-on failed."),
        DriverStage::DeadMmio => Some(b"Wi-Fi radio: no response after power-on."),
        DriverStage::FirmwareFailed => Some(b"Wi-Fi radio: firmware load failed."),
        DriverStage::NoDma => Some(b"Wi-Fi radio: DMA setup failed."),
        DriverStage::EfuseFailed => Some(b"Wi-Fi radio: efuse read failed."),
        DriverStage::NoStationAddress => Some(b"Wi-Fi radio: no entropy for a station address."),
        DriverStage::Unknown => Some(b"Wi-Fi radio: unknown driver state."),
    }
}

// The efuse registers behind an efuse-stage failure, one line under the message.
// The stage name says the read stalled; these say whether the register window is
// dead (all-ones), unclocked (all-zero), or live with the ready bit never set,
// which are three different faults with the same name on a machine that has no
// serial console to ask.
fn paint_efuse_registers(fb: &mut PaintBuffer, state: &State, stage: DriverStage, y: u32) {
    if stage != DriverStage::EfuseFailed {
        return;
    }
    let Some(dp) = state.wifi_datapath else {
        return;
    };
    // Which window the registers were read from, always, even when the read
    // itself never stalled. rtw88 uses BAR 2 on this chip; another index means
    // the reads landed somewhere the registers do not live.
    let mut bar = *b"bar=_ va=________";
    bar[4] = if dp.bar_index > 9 { b'?' } else { b'0' + dp.bar_index as u8 };
    for (i, byte) in bar[9..17].iter_mut().rev().enumerate() {
        *byte = HEX[((dp.window_va >> (i * 4)) & 0xF) as usize];
    }
    fb.text(LABEL_LEFT, y + ROW_H, &bar, STATUS_FG_IDLE);
    // The driver stores one past the stalled address, so zero means no read ever
    // stalled and the failure is upstream of the polling loop entirely.
    let Some(addr) = dp.efuse_addr.checked_sub(1) else {
        fb.text(LABEL_LEFT, y, b"efuse: no stalled read recorded", STATUS_FG_IDLE);
        return;
    };
    let mut line = [0u8; 48];
    let mut o = 0;
    for (label, value) in [(&b"ctl="[..], dp.efuse_ctl), (b"addr=", addr), (b"ldo=", dp.efuse_ldo)]
    {
        line[o..o + label.len()].copy_from_slice(label);
        o += label.len();
        for i in (0..8).rev() {
            line[o] = HEX[((value >> (i * 4)) & 0xF) as usize];
            o += 1;
        }
        line[o] = b' ';
        o += 1;
    }
    fb.text(LABEL_LEFT, y, &line[..o], STATUS_FG_IDLE);
}

// Write `v` as four lowercase hex digits into `out`.
fn write_hex4(v: u16, out: &mut [u8]) -> usize {
    const D: &[u8; 16] = b"0123456789abcdef";
    for i in 0..4 {
        out[i] = D[((v >> (12 - i * 4)) & 0xF) as usize];
    }
    4
}

// Write `n` as a signed decimal into `out`, returning how many bytes it took.
fn write_signed(n: i32, out: &mut [u8]) -> usize {
    if n < 0 {
        out[0] = b'-';
        1 + write_dec(n.unsigned_abs(), &mut out[1..])
    } else {
        write_dec(n as u32, out)
    }
}

// Write `n` as decimal into `out`, returning how many bytes it took.
pub(super) fn write_dec(mut n: u32, out: &mut [u8]) -> usize {
    if n == 0 {
        out[0] = b'0';
        return 1;
    }
    let mut digits = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    for (j, d) in digits[..i].iter().rev().enumerate() {
        out[j] = *d;
    }
    i
}

pub fn paint_wifi(fb: &mut PaintBuffer, state: &State) {
    fb.text(LABEL_LEFT, BODY_TOP + 2, b"Wi-Fi networks", STATUS_FG_IDLE);
    let top = BODY_TOP + ROW_H;

    // The radio in use, or a plain statement that there is none.
    if state.wifi_adapter_count == 0 {
        fb.text(LABEL_LEFT, top, b"No Wi-Fi adapter detected", LABEL_FG);
    } else {
        let a = &state.wifi_adapters[0];
        fb.text(LABEL_LEFT, top, a.name(), LABEL_FG);
        // "10EC:C821 h0011": the PCI id and the broker handle the driver claims.
        let mut id = *b"0000:0000 h0000";
        let (mut v, mut d) = ([0u8; 4], [0u8; 4]);
        hex4(a.vendor, &mut v);
        hex4(a.device, &mut d);
        id[..4].copy_from_slice(&v);
        id[5..9].copy_from_slice(&d);
        for (i, byte) in id[11..15].iter_mut().rev().enumerate() {
            *byte = HEX[((a.device_id >> (i * 4)) & 0xF) as usize];
        }
        fb.text(SECURED_COL, top, &id, STATUS_FG_IDLE);
    }

    // The networks the scan found.
    let net_top = top + ROW_H + 4;
    for i in 0..state.wifi_network_count {
        let net = &state.wifi_networks[i];
        let y = net_top + i as u32 * ROW_H;
        let selected = i == state.wifi_cursor;
        if selected {
            fb.fill_rect(0, y, fb.width, ROW_H, ROW_SELECTED_BG);
        }
        let fg = if selected { VALUE_FG } else { LABEL_FG };
        fb.text(LABEL_LEFT, y + 5, net.ssid(), fg);
        if net.secured {
            fb.text(SECURED_COL, y + 5, b"[secured]", STATUS_FG_IDLE);
        }
    }

    // The status line, always present, one row below the list.
    let rows = state.wifi_network_count.max(1) as u32;
    let status_y = net_top + rows * ROW_H + 8;
    paint_status(fb, state, status_y);
}

// Render the driver's receive-pipeline counters on one line: "rx: passes=N
// frames=N beacons=N". This is the diagnostic for an empty scan on a machine
// whose boot console the desktop hides.
fn paint_rx_stats(fb: &mut PaintBuffer, state: &State, y: u32) {
    let s = &state.wifi_stats;
    let mut line = [b' '; 48];
    let mut o = 0;
    let put = |bytes: &[u8], line: &mut [u8; 48], o: &mut usize| {
        for &c in bytes {
            if *o < line.len() {
                line[*o] = c;
                *o += 1;
            }
        }
    };
    put(b"rx: passes=", &mut line, &mut o);
    o += write_dec(s.steps, &mut line[o..]);
    put(b" frames=", &mut line, &mut o);
    o += write_dec(s.raw, &mut line[o..]);
    put(b" beacons=", &mut line, &mut o);
    o += write_dec(s.beacons, &mut line[o..]);
    fb.text(LABEL_LEFT, y, &line[..o], STATUS_FG_IDLE);
}

// State the scan outcome plainly. A driver that answered but is not ready reports
// the bring-up stage, so the user sees "power-on failed" rather than an empty
// list. Otherwise a completed scan reports the count, and a distinct line covers
// an empty result or a driver that could not be reached.
fn paint_status(fb: &mut PaintBuffer, state: &State, y: u32) {
    // The passphrase editor takes over the status area while it is open. The
    // typed characters are shown in the clear so a long WPA2 passphrase can be
    // checked before joining; this is a personal machine, not a shared terminal.
    if state.wifi_pass_active {
        let mut line = [b' '; 96];
        let mut o = 0;
        for &c in b"Password: " {
            line[o] = c;
            o += 1;
        }
        for &c in state.wifi_pass.as_slice() {
            if o < line.len() - 20 {
                line[o] = c;
                o += 1;
            }
        }
        for &c in b"_  [Enter joins]" {
            if o < line.len() {
                line[o] = c;
                o += 1;
            }
        }
        fb.text(LABEL_LEFT, y, &line[..o], VALUE_FG);
        return;
    }
    // The result of the last connection attempt, once one has been made.
    match state.wifi_connect {
        WifiConnect::Connected => {
            super::paint_wifi_link::paint_connected(fb, state, y);
            return;
        }
        WifiConnect::Failed(r) => {
            // Show the driver's code and the handshake progress so a failed join is
            // diagnosable on screen: -2 no such network, -5 refused, -6 timed out
            // (sent=1 recv=0 => transmit not reaching the AP), -100/-101 unreachable.
            let mut line = [b' '; 128];
            let mut o = 0;
            let put = |s: &[u8], line: &mut [u8; 128], o: &mut usize| {
                for &c in s {
                    if *o < line.len() {
                        line[*o] = c;
                        *o += 1;
                    }
                }
            };
            put(b"fail code ", &mut line, &mut o);
            o += write_signed(r.code, &mut line[o..]);
            put(b" sent=", &mut line, &mut o);
            o += write_dec(r.sent, &mut line[o..]);
            put(b" recv=", &mut line, &mut o);
            o += write_dec(r.recv, &mut line[o..]);
            put(b" data=", &mut line, &mut o);
            o += write_dec(r.data, &mut line[o..]);
            put(b" eap=", &mut line, &mut o);
            o += write_dec(r.eapol, &mut line[o..]);
            put(b" st=", &mut line, &mut o);
            o += write_dec(r.state as u32, &mut line[o..]);
            // The first stray data frame: its frame-control (hex, so the protected
            // bit and subtype read directly) and length.
            put(b" fc=", &mut line, &mut o);
            o += write_hex4((r.probe & 0xFFFF) as u16, &mut line[o..]);
            put(b" ln=", &mut line, &mut o);
            o += write_dec(r.probe >> 16, &mut line[o..]);
            // Deauth frames from the AP (it started then tore down = it sent EAPOL
            // we never answered) and data frames unicast to our own MAC (the EAPOL
            // arrives that way): these say whether the handshake frame is reaching
            // the radio at all.
            put(b" dth=", &mut line, &mut o);
            o += write_dec(r.deauth, &mut line[o..]);
            put(b" us=", &mut line, &mut o);
            o += write_dec(r.to_us, &mut line[o..]);
            fb.text(LABEL_LEFT, y, &line[..o], VALUE_FG);
            return;
        }
        WifiConnect::Idle => {}
    }
    // A radio that came up short is the reason there are no networks; say so.
    if let Some(stage) = state.wifi_stage {
        if let Some(msg) = stage_message(stage) {
            fb.text(LABEL_LEFT, y, msg, STATUS_FG_IDLE);
            paint_efuse_registers(fb, state, stage, y + ROW_H);
            return;
        }
    }
    match state.wifi_scan {
        WifiScan::Idle => {
            fb.text(LABEL_LEFT, y, b"Press Enter to scan for networks.", STATUS_FG_IDLE);
        }
        WifiScan::Done(ScanOutcome::NoService) => {
            fb.text(LABEL_LEFT, y, b"No Wi-Fi driver service is running.", STATUS_FG_IDLE);
        }
        WifiScan::Done(ScanOutcome::NoResponse) => {
            fb.text(LABEL_LEFT, y, b"The Wi-Fi driver did not respond.", STATUS_FG_IDLE);
        }
        WifiScan::Done(ScanOutcome::Scanned) => {
            if state.wifi_network_count == 0 {
                fb.text(LABEL_LEFT, y, b"No networks found. Enter to rescan.", STATUS_FG_IDLE);
                // Show the driver's receive-pipeline counters so an empty result is
                // diagnosable on screen: no scan passes means the radio never got to
                // listen, passes with no frames means the ring is delivering
                // nothing, frames with no beacons means they are not parsing.
                paint_rx_stats(fb, state, y + ROW_H);
            } else {
                let mut line = [b' '; 64];
                let mut o = 0;
                for &c in b"Found " {
                    line[o] = c;
                    o += 1;
                }
                o += write_dec(state.wifi_network_count as u32, &mut line[o..]);
                // Two keys, spelled out: c joins the highlighted network, Enter
                // starts a fresh scan.
                for &c in b" networks.  c = connect,  Enter = rescan" {
                    line[o] = c;
                    o += 1;
                }
                fb.text(LABEL_LEFT, y, &line[..o], STATUS_FG_IDLE);
            }
        }
    }
}
