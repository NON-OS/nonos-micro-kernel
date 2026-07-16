// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the association radio glue (`assoc`): a management frame is
//! classified as management, an EAPOL-carrying data frame is unwrapped to its
//! payload, anything else is ignored, an EAPOL payload wraps into an 802.11 data
//! frame addressed to the AP, and a join against a silent radio times out rather
//! than hanging.

use crate::assoc::{classify, run, wrap_eapol, Outcome, Radio, RxKind};
use nonos_wifi_core::dot11::data::{build_data, parse_data};

const OUR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0xAA];

#[test]
fn a_management_frame_is_classified_as_management() {
    // Frame control type bits = management (0). A beacon frame control is 0x0080.
    let frame = [0x80u8, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut eth = Vec::new();
    assert_eq!(classify(&frame, &mut eth), RxKind::Mgmt);
}

#[test]
fn an_eapol_data_frame_is_unwrapped_to_its_payload() {
    // Build an ethernet frame carrying an EAPOL payload, wrap it as an 802.11 data
    // frame, and confirm classify pulls the payload back out.
    let eapol = [0x02u8, 0x03, 0x00, 0x5F, 0xAB, 0xCD]; // EAPOL version/type + body
    let mut eth = Vec::new();
    eth.extend_from_slice(&OUR); // dst
    eth.extend_from_slice(&AP); // src
    eth.extend_from_slice(&0x888Eu16.to_be_bytes());
    eth.extend_from_slice(&eapol);
    let mpdu = build_data(&eth, OUR, AP, 0).unwrap();

    let mut out = Vec::new();
    match classify(&mpdu, &mut out) {
        RxKind::Eapol(start, end) => assert_eq!(&out[start..end], &eapol),
        other => panic!("expected EAPOL, got {other:?}"),
    }
}

#[test]
fn a_non_eapol_data_frame_is_ignored() {
    // A data frame carrying IPv4 (ethertype 0x0800) is not part of the join.
    let mut eth = Vec::new();
    eth.extend_from_slice(&OUR);
    eth.extend_from_slice(&AP);
    eth.extend_from_slice(&0x0800u16.to_be_bytes());
    eth.extend_from_slice(&[1, 2, 3, 4]);
    let mpdu = build_data(&eth, OUR, AP, 0).unwrap();
    let mut out = Vec::new();
    assert_eq!(classify(&mpdu, &mut out), RxKind::Other);
}

#[test]
fn wrap_eapol_builds_a_data_frame_to_the_ap() {
    let eapol = [0x02u8, 0x03, 0x00, 0x10, 0x99];
    let mpdu = wrap_eapol(&eapol, OUR, AP, 7).unwrap();
    // It round-trips: parse the data frame back to ethernet and recover the EAPOL.
    let eth = parse_data(&mpdu).unwrap();
    assert_eq!(&eth[0..6], &AP, "destination is the AP");
    assert_eq!(&eth[6..12], &OUR, "source is the station");
    assert_eq!(u16::from_be_bytes([eth[12], eth[13]]), 0x888E, "ethertype is EAPOL");
    assert_eq!(&eth[14..], &eapol, "payload preserved");
}

// A radio that never delivers a frame, to prove the join gives up instead of
// spinning forever.
struct SilentRadio {
    sent: usize,
}
impl Radio for SilentRadio {
    fn send(&mut self, _mpdu: &[u8]) -> bool {
        self.sent += 1;
        true
    }
    fn recv(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
}

#[test]
fn a_join_against_a_silent_radio_times_out() {
    let mut radio = SilentRadio { sent: 0 };
    // A minimal beacon for "net" with an RSN element, enough for the machine to
    // select the BSS and emit the auth request (which the radio swallows).
    let beacon = beacon_for(b"net");
    let report = run(&mut radio, OUR, b"net", b"password123", &beacon, [0x11u8; 32], 32);
    assert!(matches!(report.outcome, Outcome::TimedOut), "a silent AP ends in a timeout");
    assert_eq!(report.sent, 1, "the auth request was transmitted from the beacon");
    assert_eq!(report.recv, 0, "a silent radio delivered nothing");
    assert_eq!(report.state, 1, "the machine reached Authenticating and waited");
    assert!(radio.sent >= 1, "the radio saw the transmit");
}

// Build a beacon frame for `ssid` advertising RSN, the shape parse_beacon accepts.
fn beacon_for(ssid: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x80, 0x00]); // frame control: beacon
    f.extend_from_slice(&[0, 0]); // duration
    f.extend_from_slice(&[0xFF; 6]); // DA broadcast
    f.extend_from_slice(&AP); // SA
    f.extend_from_slice(&AP); // BSSID
    f.extend_from_slice(&[0, 0]); // seq
    f.extend_from_slice(&[0u8; 8]); // timestamp
    f.extend_from_slice(&[0x64, 0]); // beacon interval
    f.extend_from_slice(&[0x11, 0x04]); // capability: ESS + privacy
    // SSID element.
    f.push(0);
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    // DS parameter set: channel 6.
    f.extend_from_slice(&[3, 1, 6]);
    // RSN element (id 48), a minimal WPA2-PSK/CCMP body.
    let rsn: [u8; 20] = [
        0x01, 0x00, // version
        0x00, 0x0F, 0xAC, 0x04, // group cipher CCMP
        0x01, 0x00, 0x00, 0x0F, 0xAC, 0x04, // pairwise CCMP
        0x01, 0x00, 0x00, 0x0F, 0xAC, 0x02, // akm PSK
        0x00, 0x00, // rsn caps
    ];
    f.push(48);
    f.push(rsn.len() as u8);
    f.extend_from_slice(&rsn);
    f
}
