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

//! The connected view: whether net_core is running and the address it holds, plus
//! the driver's TX/RX frame counts, so a connection that associated but never got
//! an address is legible on a serial-less laptop.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::state::State;
use crate::settings::theme::{STATUS_FG_IDLE, VALUE_FG};
use crate::wifi::NetStatus;

use super::fmt_ip::write_ip;
use super::layout::{LABEL_LEFT, ROW_H};
use super::paint_wifi::write_dec;

/// Paint the connected line: the net_core state (down, bound-without-address, or a
/// bound address), then the driver's frame counts below.
pub fn paint_connected(fb: &mut PaintBuffer, state: &State, y: u32) {
    match state.wifi_net {
        NetStatus::Bound { lease, .. } => {
            let mut line = [b' '; 40];
            let mut o = put(&mut line, 0, b"Connected.  IP ");
            o += write_ip(&lease.ip, &mut line[o..]);
            fb.text(LABEL_LEFT, y, &line[..o], VALUE_FG);
        }
        NetStatus::Unbound { port } => {
            let mut line = [b' '; 48];
            let mut o = put(&mut line, 0, b"Connected.  Net stack up, no address (nic ");
            o += write_dec(port, &mut line[o..]);
            o = put(&mut line, o, b")");
            fb.text(LABEL_LEFT, y, &line[..o], VALUE_FG);
        }
        NetStatus::NoService => {
            fb.text(LABEL_LEFT, y, b"Connected.  DHCP service not registered.", VALUE_FG);
        }
        NetStatus::NoReply => {
            fb.text(LABEL_LEFT, y, b"Connected.  Waiting for net stack reply...", VALUE_FG);
        }
    }
    if let Some(d) = state.wifi_datapath {
        let mut line = [b' '; 80];
        let mut o = put(&mut line, 0, b"tx ");
        o += write_dec(d.tx_ok, &mut line[o..]);
        o = put(&mut line, o, b"/");
        o += write_dec(d.tx_drop, &mut line[o..]);
        o = put(&mut line, o, b"  rx ");
        o += write_dec(d.rx_ring, &mut line[o..]);
        o = put(&mut line, o, b"/");
        o += write_dec(d.rx_eth, &mut line[o..]);
        o = put(&mut line, o, b"  net ");
        o += write_dec(d.netif_reqs, &mut line[o..]);
        o = put(&mut line, o, b"  err ");
        o += write_dec(d.rx_err, &mut line[o..]);
        fb.text(LABEL_LEFT, y + ROW_H, &line[..o], STATUS_FG_IDLE);
    }
}

// Copy `src` into `line` at `at`, returning the new offset.
fn put(line: &mut [u8], at: usize, src: &[u8]) -> usize {
    let n = src.len().min(line.len().saturating_sub(at));
    line[at..at + n].copy_from_slice(&src[..n]);
    at + n
}
