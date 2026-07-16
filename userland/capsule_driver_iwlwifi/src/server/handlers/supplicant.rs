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

//! The WPA2 4-way handshake exposed to a connection manager. START opens a
//! handshake from the PMK, MACs and SNonce; STEP feeds each EAPOL-Key frame
//! the AP sent and returns the reply frame to transmit plus the handshake
//! state. The keys land in the driver's supplicant once state is Connected.

use crate::driver::Driver;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;
use nonos_wifi_core::wpa::supplicant::{State, Supplicant};

const START_LEN: usize = 32 + 6 + 6 + 32;

/// Body: pmk(32) || aa(6) || spa(6) || snonce(32). Reply: one state byte.
pub fn start(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() != START_LEN {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let mut pmk = [0u8; 32];
    let mut aa = [0u8; 6];
    let mut spa = [0u8; 6];
    let mut snonce = [0u8; 32];
    pmk.copy_from_slice(&body[0..32]);
    aa.copy_from_slice(&body[32..38]);
    spa.copy_from_slice(&body[38..44]);
    snonce.copy_from_slice(&body[44..76]);
    let sup = Supplicant::new(pmk, aa, spa, snonce);
    let state = state_byte(sup.state());
    driver.supplicant = Some(sup);
    let _ = respond::send(sender_pid, req, E_OK, &[state], out);
}

/// Body: the received EAPOL-Key frame. Reply: state byte, then the reply frame
/// to transmit (empty when there is nothing to send).
pub fn step(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    let Some(sup) = driver.supplicant.as_mut() else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    let result = sup.step(body);
    let state = sup.state();
    // Reply: state byte, TK length, TK, GTK length, GTK, then the frame to
    // transmit. The keys are non-empty only once Connected, giving the
    // connection manager everything it needs to install them and send the
    // final handshake message in one round-trip.
    let mut reply = alloc::vec::Vec::new();
    reply.push(state_byte(state));
    let (tk, gtk) = if state == State::Connected { (sup.tk(), sup.gtk()) } else { (&[][..], &[][..]) };
    reply.push(tk.len() as u8);
    reply.extend_from_slice(tk);
    reply.push(gtk.len() as u8);
    reply.extend_from_slice(gtk);
    if let Some(frame) = result.reply {
        reply.extend_from_slice(&frame);
    }
    let _ = respond::send(sender_pid, req, E_OK, &reply, out);
}

fn state_byte(s: State) -> u8 {
    match s {
        State::Start => 0,
        State::PtkDerived => 1,
        State::Connected => 2,
        State::Failed => 0xFF,
    }
}
