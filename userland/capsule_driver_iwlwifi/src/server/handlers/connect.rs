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

//! The association MLME exposed to a connection manager. START opens an
//! association to an SSID with a passphrase; MGMT and EAPOL feed received
//! frames and return the frame to transmit next plus the link state. When the
//! link comes up the reply carries the pairwise and group keys.

use crate::driver::Driver;
use nonos_wifi_core::mlme::{Mlme, MlmeOutput, MlmeState};
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

/// Body: our_mac(6) || snonce(32) || ssid_len(1) || ssid || passphrase.
pub fn start(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < 39 {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let mut our_mac = [0u8; 6];
    let mut snonce = [0u8; 32];
    our_mac.copy_from_slice(&body[0..6]);
    snonce.copy_from_slice(&body[6..38]);
    let ssid_len = body[38] as usize;
    let ssid_end = 39 + ssid_len;
    if body.len() < ssid_end {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let ssid = &body[39..ssid_end];
    let passphrase = &body[ssid_end..];
    let mlme = Mlme::new(our_mac, ssid, passphrase, snonce);
    let state = state_byte(mlme.state());
    driver.mlme = Some(mlme);
    let _ = respond::send(sender_pid, req, E_OK, &[state], out);
}

/// Body: a received 802.11 management frame. Reply: state byte then TX frame.
pub fn mgmt(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    feed(driver, sender_pid, req, out, |m| m.on_mgmt(body));
}

/// Body: a received EAPOL-Key payload. Reply: state byte, keys once connected,
/// then the TX frame.
pub fn eapol(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    feed(driver, sender_pid, req, out, |m| m.on_eapol(body));
}

fn feed(
    driver: &mut Driver,
    sender_pid: u32,
    req: &Request,
    out: &mut [u8],
    run: impl FnOnce(&mut Mlme) -> MlmeOutput,
) {
    let Some(mlme) = driver.mlme.as_mut() else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    let result = run(mlme);
    let state = mlme.state();
    let mut reply = alloc::vec::Vec::new();
    reply.push(state_byte(state));
    // The channel the beacon named: the driver tunes its radio here before the
    // auth/assoc exchange the returned frame belongs to.
    reply.push(mlme.channel());
    let tk = mlme.tk().unwrap_or(&[]);
    let gtk = mlme.gtk().unwrap_or(&[]);
    reply.push(tk.len() as u8);
    reply.extend_from_slice(tk);
    reply.push(gtk.len() as u8);
    reply.extend_from_slice(gtk);
    if let Some(frame) = result.tx {
        reply.extend_from_slice(&frame);
    }
    let _ = respond::send(sender_pid, req, E_OK, &reply, out);
}

fn state_byte(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 0xFF,
    }
}
