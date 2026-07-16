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

//! Drive the association forward. Management frames (beacon, auth response,
//! association response) advance scan -> authenticate -> associate; EAPOL
//! frames run the four-way handshake through the supplicant.

use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::mgmt::{assoc_request, auth_open};
use crate::dot11::parse::{parse_assoc_response, parse_auth_status, parse_beacon};
use crate::wpa::supplicant::{State as SupState, Supplicant};

// Capability: ESS + Privacy (a WPA2 network advertises privacy). Rates: the
// 802.11g set, basic bit set on the mandatory ones.
const CAP_ESS_PRIVACY: u16 = 0x0011;
const RATES: [u8; 8] = [0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];

impl Mlme {
    /// Feed an 802.11 management frame received while joining.
    pub fn on_mgmt(&mut self, frame: &[u8]) -> MlmeOutput {
        match self.state {
            MlmeState::Scanning => self.on_beacon(frame),
            MlmeState::Authenticating => self.on_auth_response(frame),
            MlmeState::Associating => self.on_assoc_response(frame),
            _ => MlmeOutput::none(),
        }
    }

    /// Feed the payload of an EAPOL-Key frame received while associated.
    pub fn on_eapol(&mut self, frame: &[u8]) -> MlmeOutput {
        if self.state != MlmeState::FourWay {
            return MlmeOutput::none();
        }
        let Some(sup) = self.supplicant.as_mut() else {
            self.state = MlmeState::Failed;
            return MlmeOutput::none();
        };
        let out = sup.step(frame);
        match sup.state() {
            SupState::Connected => {
                self.state = MlmeState::Connected;
                MlmeOutput { tx: out.reply }
            }
            SupState::Failed => {
                self.state = MlmeState::Failed;
                MlmeOutput::none()
            }
            _ => MlmeOutput { tx: out.reply },
        }
    }

    // A beacon for the target SSID that advertises RSN selects the BSS and
    // starts authentication.
    fn on_beacon(&mut self, frame: &[u8]) -> MlmeOutput {
        let Some(info) = parse_beacon(frame) else {
            return MlmeOutput::none();
        };
        if info.ssid != self.ssid() || !info.rsn {
            return MlmeOutput::none();
        }
        self.bssid = info.bssid;
        self.channel = info.channel;
        let seq = self.next_seq();
        let mut out = [0u8; 64];
        match auth_open(&mut out, self.our_mac, self.bssid, seq) {
            Some(n) => {
                self.state = MlmeState::Authenticating;
                MlmeOutput { tx: Some(out[..n].to_vec()) }
            }
            None => self.fail(),
        }
    }

    // A successful open-auth response moves to association.
    fn on_auth_response(&mut self, frame: &[u8]) -> MlmeOutput {
        match parse_auth_status(frame) {
            Some(0) => {
                let seq = self.next_seq();
                let mut out = [0u8; 128];
                let ssid = self.ssid;
                let len = self.ssid_len;
                match assoc_request(
                    &mut out,
                    self.our_mac,
                    self.bssid,
                    &ssid[..len],
                    &RATES,
                    &crate::wpa::RSN_IE,
                    CAP_ESS_PRIVACY,
                    seq,
                ) {
                    Some(n) => {
                        self.state = MlmeState::Associating;
                        MlmeOutput { tx: Some(out[..n].to_vec()) }
                    }
                    None => self.fail(),
                }
            }
            Some(_) => self.fail(),
            None => MlmeOutput::none(),
        }
    }

    // A successful association response starts the four-way handshake; the AP
    // sends message 1 next, which arrives via on_eapol.
    fn on_assoc_response(&mut self, frame: &[u8]) -> MlmeOutput {
        match parse_assoc_response(frame) {
            Some((0, _aid)) => {
                self.supplicant =
                    Some(Supplicant::new(self.pmk, self.bssid, self.our_mac, self.snonce));
                self.state = MlmeState::FourWay;
                MlmeOutput::none()
            }
            Some(_) => self.fail(),
            None => MlmeOutput::none(),
        }
    }

    fn fail(&mut self) -> MlmeOutput {
        self.state = MlmeState::Failed;
        MlmeOutput::none()
    }
}
