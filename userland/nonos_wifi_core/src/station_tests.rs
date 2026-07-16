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

//! Proofs for the shared station data-path engine: a hardware-CCMP station
//! frames plaintext, a software-CCMP station encrypts, both recover the frame,
//! the counters advance, and an unassociated station transmits nothing. No
//! hardware; the AES-CCM core these rest on is proven against RFC 3610 elsewhere.

use crate::station::{Ccmp, LinkStation};

const OUR: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
const BSSID: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const DST: [u8; 6] = [0x33, 0x33, 0x00, 0x00, 0x00, 0x16];
const TK: [u8; 16] = [
    0xc9, 0x7c, 0x1f, 0x67, 0xce, 0x37, 0x11, 0x85, 0x51, 0x4a, 0x8a, 0x19, 0xf2, 0xbd, 0xd5, 0x2f,
];
const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
const FC_PROTECTED: u16 = 0x4000;

fn eth() -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&DST);
    f.extend_from_slice(&OUR); // the stack sends with our MAC as source
    f.extend_from_slice(&[0x08, 0x00]);
    f.extend_from_slice(b"a frame that rides the radio");
    f
}

#[test]
fn an_unassociated_station_transmits_nothing() {
    let mut sta = LinkStation::new(OUR);
    assert!(!sta.is_associated());
    assert!(sta.tx_frame(&eth()).is_none(), "no framing before association");
}

#[test]
fn a_hardware_station_frames_plaintext_and_recovers_it() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    let mpdu = sta.tx_frame(&eth()).expect("tx");

    // The radio does the crypto, so the station must hand it a plaintext frame.
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    assert_eq!(fc & FC_PROTECTED, 0, "hardware CCMP frame is not marked encrypted");
    assert_eq!(&mpdu[24..30], &LLC_SNAP, "body is plaintext LLC/SNAP");

    // The receive path (chip already decrypted) recovers the ethernet frame.
    let back = sta.rx_frame(&mpdu).expect("rx");
    assert_eq!(back, eth(), "the ethernet frame survives the hardware round trip");
}

#[test]
fn a_software_station_encrypts_and_recovers_it() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Software { tk: TK });
    let frame = sta.tx_frame(&eth()).expect("tx");

    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    assert_ne!(fc & FC_PROTECTED, 0, "software CCMP marks the frame protected");
    assert_ne!(&frame[32..38], &LLC_SNAP, "the body is ciphertext, not plaintext LLC/SNAP");

    let back = sta.rx_frame(&frame).expect("rx");
    assert_eq!(back, eth(), "the ethernet frame survives the software round trip");
}

#[test]
fn the_sequence_and_packet_numbers_advance_each_frame() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Software { tk: TK });
    let first = sta.tx_frame(&eth()).expect("tx1");
    let second = sta.tx_frame(&eth()).expect("tx2");

    // Sequence-control high 12 bits carry the sequence number: 0 then 1.
    let seq1 = u16::from_le_bytes([first[22], first[23]]) >> 4;
    let seq2 = u16::from_le_bytes([second[22], second[23]]) >> 4;
    assert_eq!((seq1, seq2), (0, 1), "the sequence number advances per frame");

    // The CCMP header's first packet-number octet advances 1 -> 2.
    assert_eq!(first[24], 1, "first packet number is one");
    assert_eq!(second[24], 2, "packet number advances, so a nonce is never reused");
    assert_ne!(first, second, "identical plaintext yields distinct frames");
}

#[test]
fn a_malformed_ethernet_frame_is_refused() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    assert!(sta.tx_frame(&[0u8; 8]).is_none(), "a runt shorter than an ethernet header is refused");
}
