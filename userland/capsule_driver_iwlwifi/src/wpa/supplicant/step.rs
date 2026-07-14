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

//! Drive one EAPOL-Key frame through the handshake. Message 1 (no MIC) yields
//! message 2; message 3 (MIC set, install) is verified and yields message 4.

use alloc::vec::Vec;

use super::state::{State, Supplicant};
use crate::ccmp::keywrap::aes_unwrap;
use crate::eapol::build::build_key_frame;
use crate::eapol::mic::verify_mic;
use crate::eapol::parse::{
    parse, KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_SECURE, KEY_INFO_VERSION2,
};
use crate::wpa::ptk::ptk;

// A WPA2-PSK-CCMP RSN element, sent as message 2's key data so the AP sees the
// pairwise/group/AKM selection the STA committed to. Group CCMP, pairwise
// CCMP, AKM PSK.
const RSN_IE: [u8; 22] = [
    0x30, 0x14, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00,
    0x00, 0x0f, 0xac, 0x02, 0x00, 0x00,
];

/// What `step` produced: a frame to transmit, and whether the handshake is now
/// complete. A frame is returned for messages 2 and 4; `None` means the input
/// was not a handshake message the supplicant acts on, or verification failed
/// (check `Supplicant::state` for `Failed`).
pub struct StepOutput {
    pub reply: Option<Vec<u8>>,
}

impl Supplicant {
    pub fn step(&mut self, frame: &[u8]) -> StepOutput {
        let Some(key) = parse(frame) else {
            return StepOutput { reply: None };
        };
        if key.key_info & KEY_INFO_PAIRWISE == 0 {
            return StepOutput { reply: None };
        }
        // Message 1 carries the ANonce with no MIC; message 3 carries a MIC.
        // That bit alone distinguishes them unambiguously in the pairwise
        // exchange, which is why the state is checked alongside it.
        let has_mic = key.key_info & KEY_INFO_MIC != 0;
        match (self.state, has_mic) {
            (State::Start, false) => self.on_message1(key.nonce, key.replay_counter),
            (State::PtkDerived, true) => self.on_message3(frame, &key),
            _ => StepOutput { reply: None },
        }
    }

    // Message 1 -> derive the PTK, answer with message 2 (SNonce + RSN IE,
    // MIC keyed by the fresh KCK).
    fn on_message1(&mut self, anonce: [u8; 32], replay: [u8; 8]) -> StepOutput {
        self.anonce = anonce;
        self.ptk = ptk(&self.pmk, &self.aa, &self.spa, &self.anonce, &self.snonce);
        let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC;
        let mut out = [0u8; 128];
        match build_key_frame(&mut out, info, &replay, &self.snonce, &RSN_IE, self.kck()) {
            Some(n) => {
                self.state = State::PtkDerived;
                StepOutput { reply: Some(out[..n].to_vec()) }
            }
            None => {
                self.state = State::Failed;
                StepOutput { reply: None }
            }
        }
    }

    // Message 3 -> verify its MIC and ANonce, unwrap the group key, answer with
    // message 4. Any mismatch abandons the handshake.
    fn on_message3(&mut self, frame: &[u8], key: &crate::eapol::parse::EapolKey<'_>) -> StepOutput {
        if !verify_mic(self.kck(), frame) || key.nonce != self.anonce {
            self.state = State::Failed;
            return StepOutput { reply: None };
        }
        if !self.install_group_key(key.key_data) {
            self.state = State::Failed;
            return StepOutput { reply: None };
        }
        let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC | KEY_INFO_SECURE;
        let empty: [u8; 0] = [];
        let mut out = [0u8; 128];
        match build_key_frame(&mut out, info, &key.replay_counter, &self.snonce, &empty, self.kck())
        {
            Some(n) => {
                self.state = State::Connected;
                StepOutput { reply: Some(out[..n].to_vec()) }
            }
            None => {
                self.state = State::Failed;
                StepOutput { reply: None }
            }
        }
    }

    // Message 3's key data is AES-key-wrapped under the KEK and holds a GTK
    // KDE (dd <len> 00 0f ac 01 <keyid> <gtk>). Unwrap it, find the KDE, and
    // store the group key.
    fn install_group_key(&mut self, wrapped: &[u8]) -> bool {
        if wrapped.len() < 16 || !wrapped.len().is_multiple_of(8) {
            return false;
        }
        let mut plain = [0u8; 128];
        let Some(len) = aes_unwrap(&self.kek(), wrapped, &mut plain) else {
            return false;
        };
        find_gtk(&plain[..len], &mut self.gtk).map(|n| self.gtk_len = n).is_some()
    }
}

// Scan an unwrapped key-data buffer for the GTK KDE and copy the key out.
// KDE: 0xDD, length, 00 0F AC (RSN OUI), 0x01 (GTK), key-id byte, reserved,
// then the group key.
fn find_gtk(data: &[u8], out: &mut [u8; 32]) -> Option<usize> {
    let mut i = 0usize;
    while i + 2 <= data.len() {
        let tag = data[i];
        let len = data[i + 1] as usize;
        let body_end = i.checked_add(2)?.checked_add(len)?;
        if body_end > data.len() {
            return None;
        }
        if tag == 0xDD
            && len >= 6
            && data[i + 2..i + 5] == [0x00, 0x0f, 0xac]
            && data[i + 5] == 0x01
        {
            let gtk = &data[i + 8..body_end];
            if gtk.is_empty() || gtk.len() > out.len() {
                return None;
            }
            out[..gtk.len()].copy_from_slice(gtk);
            return Some(gtk.len());
        }
        if tag == 0x00 {
            break; // padding
        }
        i = body_end;
    }
    None
}
