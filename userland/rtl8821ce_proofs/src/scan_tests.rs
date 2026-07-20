// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the scan result collector: it deduplicates by access point, drops
//! hidden networks, caps the list, and encodes in the exact `[count]` then
//! per-network `[signal][flags][ssid_len][ssid]` format the settings panel
//! parses. A real beacon, decoded by the shared `parse_beacon`, becomes a result
//! end to end. The channel-hop and ring reads are hardware timing left for the
//! on-silicon session; the collection and encoding are checked here.

use crate::scan::{ScanResults, MAX_RESULTS};
use nonos_wifi_core::dot11::parse::parse_beacon;

// Build a beacon frame that starts at the MAC header: broadcast destination, the
// given BSSID, the fixed fields, an SSID element and, when secured, an RSN one.
fn beacon(bssid: [u8; 6], ssid: &[u8], secured: bool) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x80, 0x00]); // frame control: beacon
    f.extend_from_slice(&[0x00, 0x00]); // duration
    f.extend_from_slice(&[0xff; 6]); // addr1: broadcast
    f.extend_from_slice(&bssid); // addr2
    f.extend_from_slice(&bssid); // addr3 (BSSID)
    f.extend_from_slice(&[0x00, 0x00]); // sequence
    f.extend_from_slice(&[0u8; 8]); // timestamp
    f.extend_from_slice(&[0x64, 0x00]); // beacon interval
    f.extend_from_slice(&[0x11, 0x04]); // capability
    f.push(0); // SSID element id
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    if secured {
        f.push(48); // RSN element id
        f.push(2);
        f.extend_from_slice(&[0x01, 0x00]);
    }
    f
}

#[test]
fn add_dedupes_by_access_point_and_skips_hidden() {
    let mut r = ScanResults::new();
    let ap = [0x02, 0, 0, 0, 0, 1];
    r.add(ap, b"Home", true);
    r.add(ap, b"Home", true); // same AP again
    r.add([0x02, 0, 0, 0, 0, 2], b"Cafe", false);
    r.add([0x02, 0, 0, 0, 0, 3], b"", false); // hidden
    assert_eq!(r.count(), 2, "the repeat and the hidden network are dropped");
}

#[test]
fn add_caps_at_the_maximum() {
    let mut r = ScanResults::new();
    for i in 0..(MAX_RESULTS as u8 + 5) {
        r.add([0x02, 0, 0, 0, 0, i], b"Net", false);
    }
    assert_eq!(r.count(), MAX_RESULTS, "the list stops at the cap");
}

#[test]
fn encode_matches_the_panel_format() {
    let mut r = ScanResults::new();
    r.add([0x02, 0, 0, 0, 0, 1], b"Home", true);
    r.add([0x02, 0, 0, 0, 0, 2], b"Cafe", false);
    let mut out = [0u8; 128];
    let n = r.encode(&mut out);
    assert_eq!(out[0], 2, "count leads");
    // First network: signal 0, flags secured, ssid_len 4, "Home".
    assert_eq!(out[1], 0, "signal byte");
    assert_eq!(out[2], 0x01, "secured flag set");
    assert_eq!(out[3], 4, "ssid length");
    assert_eq!(&out[4..8], b"Home");
    // Second network follows immediately: signal, flags clear, len 4, "Cafe".
    assert_eq!(out[8], 0, "signal byte");
    assert_eq!(out[9], 0x00, "open network flag clear");
    assert_eq!(out[10], 4);
    assert_eq!(&out[11..15], b"Cafe");
    assert_eq!(n, 15, "count byte plus two 7-byte entries");
}

#[test]
fn a_parsed_beacon_becomes_a_result() {
    let frame = beacon([0x02, 0xAB, 0xCD, 0xEF, 0x00, 0x01], b"TestNet", true);
    let info = parse_beacon(&frame).expect("the beacon parses");
    let mut r = ScanResults::new();
    r.add(info.bssid, info.ssid, info.rsn);
    assert_eq!(r.count(), 1);
    let mut out = [0u8; 64];
    r.encode(&mut out);
    assert_eq!(out[0], 1);
    assert_eq!(out[2] & 0x01, 0x01, "the RSN beacon is reported secured");
    assert_eq!(out[3], 7, "SSID length of TestNet");
    assert_eq!(&out[4..11], b"TestNet");
}
