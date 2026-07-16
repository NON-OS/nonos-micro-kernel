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

//! The encrypted data path once the link is up. TX takes an ethernet frame
//! from the net stack and returns the CCMP-encrypted 802.11 frame to put on the
//! air; RX takes a received 802.11 data frame and returns the decrypted
//! ethernet frame. The pairwise key comes from the completed handshake.

use crate::dot11::data::{decap, encap};
use crate::driver::Driver;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

/// Body: an ethernet frame. Reply: the CCMP-encrypted 802.11 data frame.
pub fn tx(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    let Some((tk, our, bssid)) = link(driver) else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    driver.tx_pn = driver.tx_pn.wrapping_add(1);
    let seq = driver.tx_pn as u16;
    match encap(body, our, bssid, driver.tx_pn, &tk, seq) {
        Some(frame) => {
            let _ = respond::send(sender_pid, req, E_OK, &frame, out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}

/// Body: a received 802.11 data frame. Reply: the decrypted ethernet frame.
pub fn rx(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    let Some((tk, _, _)) = link(driver) else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    match decap(body, &tk) {
        Some(eth) => {
            let _ = respond::send(sender_pid, req, E_OK, &eth, out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}

// The pairwise key and addresses of a connected link, or None when not yet
// connected.
fn link(driver: &Driver) -> Option<([u8; 16], [u8; 6], [u8; 6])> {
    let mlme = driver.mlme.as_ref()?;
    let tk_slice = mlme.tk()?;
    if tk_slice.len() != 16 {
        return None;
    }
    let mut tk = [0u8; 16];
    tk.copy_from_slice(tk_slice);
    Some((tk, mlme.our_mac(), mlme.bssid()))
}
