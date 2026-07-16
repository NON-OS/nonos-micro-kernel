// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the 802.11 frame layer. The encoding is the IEEE standard, so
//! these check the exact bytes the builders lay down: frame-control fields,
//! the 24-byte header with addresses in place, and the management-frame bodies
//! and information elements a scan, an auth and an association are made of.

use crate::dot11::header::{
    fc_subtype, fc_type, frame_control, seq_control, write_header, BROADCAST, MAC_HEADER_LEN,
    SUBTYPE_ASSOC_REQ, SUBTYPE_AUTH, SUBTYPE_BEACON, SUBTYPE_PROBE_REQ, TYPE_MGMT,
};
use crate::dot11::mgmt::{assoc_request, auth_open, probe_request, IE_SSID, IE_SUPPORTED_RATES};
use crate::dot11::parse::parse_beacon;

#[test]
fn frame_control_roundtrips_type_and_subtype() {
    for t in 0..4u8 {
        for s in 0..16u8 {
            let fc = frame_control(t, s);
            assert_eq!(fc_type(fc), t);
            assert_eq!(fc_subtype(fc), s);
            assert_eq!(fc & 0x3, 0, "protocol version is always zero");
        }
    }
}

#[test]
fn seq_control_places_the_12_bit_sequence() {
    for seq in [0u16, 1, 100, 0x0FFF, 0x1000, 0xFFFF] {
        let sc = seq_control(seq);
        assert_eq!(sc & 0xF, 0, "fragment number is zero");
        assert_eq!((sc >> 4) & 0x0FFF, seq & 0x0FFF, "the 12-bit sequence is preserved");
    }
}

#[test]
fn header_is_24_bytes_with_addresses_in_place() {
    let mut buf = [0u8; 64];
    let src = [0x02, 0, 0, 0, 0, 0x01];
    let bssid = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_PROBE_REQ);
    let n = write_header(&mut buf, fc, BROADCAST, src, bssid, 7).unwrap();
    assert_eq!(n, MAC_HEADER_LEN);
    assert_eq!(&buf[0..2], &fc.to_le_bytes());
    assert_eq!(&buf[4..10], &BROADCAST, "addr1 is the receiver");
    assert_eq!(&buf[10..16], &src, "addr2 is the transmitter");
    assert_eq!(&buf[16..22], &bssid, "addr3 is the BSSID");
}

#[test]
fn write_header_rejects_a_short_buffer() {
    let mut small = [0u8; 20];
    assert!(write_header(&mut small, 0, BROADCAST, BROADCAST, BROADCAST, 0).is_none());
}

#[test]
fn probe_request_is_a_broadcast_mgmt_frame_with_ssid_and_rates() {
    let mut buf = [0u8; 128];
    let src = [0x02, 0, 0, 0, 0, 0x01];
    let ssid = b"nonos-net";
    let rates = &[0x82u8, 0x84, 0x8b, 0x96];
    let n = probe_request(&mut buf, src, ssid, rates, 1).unwrap();
    assert_eq!(n, MAC_HEADER_LEN + (2 + ssid.len()) + (2 + rates.len()));
    let fc = u16::from_le_bytes([buf[0], buf[1]]);
    assert_eq!(fc_type(fc), TYPE_MGMT);
    assert_eq!(fc_subtype(fc), SUBTYPE_PROBE_REQ);
    assert_eq!(&buf[4..10], &BROADCAST, "a probe request is broadcast");
    assert_eq!(buf[MAC_HEADER_LEN], IE_SSID);
    assert_eq!(buf[MAC_HEADER_LEN + 1] as usize, ssid.len());
    assert_eq!(&buf[MAC_HEADER_LEN + 2..MAC_HEADER_LEN + 2 + ssid.len()], ssid);
    let rates_off = MAC_HEADER_LEN + 2 + ssid.len();
    assert_eq!(buf[rates_off], IE_SUPPORTED_RATES);
}

#[test]
fn auth_open_body_is_algorithm_zero_transaction_one() {
    let mut buf = [0u8; 64];
    let src = [0x02, 0, 0, 0, 0, 0x01];
    let bssid = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let n = auth_open(&mut buf, src, bssid, 3).unwrap();
    assert_eq!(n, MAC_HEADER_LEN + 6);
    let fc = u16::from_le_bytes([buf[0], buf[1]]);
    assert_eq!(fc_subtype(fc), SUBTYPE_AUTH);
    assert_eq!(u16::from_le_bytes([buf[MAC_HEADER_LEN], buf[MAC_HEADER_LEN + 1]]), 0, "open system");
    assert_eq!(u16::from_le_bytes([buf[MAC_HEADER_LEN + 2], buf[MAC_HEADER_LEN + 3]]), 1, "transaction 1");
    assert_eq!(&buf[4..10], &bssid, "a directed auth is addressed to the AP");
}

#[test]
fn assoc_request_carries_capability_and_ies() {
    let mut buf = [0u8; 128];
    let src = [0x02, 0, 0, 0, 0, 0x01];
    let bssid = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let ssid = b"nonos-net";
    let rates = &[0x82u8, 0x84];
    let n = assoc_request(&mut buf, src, bssid, ssid, rates, 0x0431, 5).unwrap();
    let fc = u16::from_le_bytes([buf[0], buf[1]]);
    assert_eq!(fc_subtype(fc), SUBTYPE_ASSOC_REQ);
    assert_eq!(u16::from_le_bytes([buf[MAC_HEADER_LEN], buf[MAC_HEADER_LEN + 1]]), 0x0431, "capability info");
    assert_eq!(n, MAC_HEADER_LEN + 4 + (2 + ssid.len()) + (2 + rates.len()));
}

#[test]
fn builders_reject_overflow() {
    let mut tiny = [0u8; MAC_HEADER_LEN + 2];
    let src = [0u8; 6];
    assert!(probe_request(&mut tiny, src, b"waytoolongssidforthisbuffer", &[0x82], 0).is_none());
}

#[test]
fn parse_beacon_extracts_ssid_channel_and_rsn() {
    let mut f = [0u8; 128];
    let bssid = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_BEACON);
    let n = write_header(&mut f, fc, BROADCAST, bssid, bssid, 0).unwrap();
    // fixed fields: timestamp(8), beacon interval(2), capability(2 at +10)
    f[n + 10] = 0x11;
    f[n + 11] = 0x04;
    let mut o = n + 12;
    let ssid = b"nonos-ap";
    f[o] = 0;
    f[o + 1] = ssid.len() as u8;
    f[o + 2..o + 2 + ssid.len()].copy_from_slice(ssid);
    o += 2 + ssid.len();
    f[o] = 3;
    f[o + 1] = 1;
    f[o + 2] = 6; // DS param: channel 6
    o += 3;
    f[o] = 48;
    f[o + 1] = 2;
    f[o + 2] = 1;
    f[o + 3] = 0; // RSN element
    o += 4;
    let info = parse_beacon(&f[..o]).unwrap();
    assert_eq!(info.bssid, bssid);
    assert_eq!(info.ssid, ssid);
    assert_eq!(info.channel, 6);
    assert!(info.rsn, "RSN (WPA2) advertised");
    assert_eq!(info.capability, 0x0411);
}

#[test]
fn parse_beacon_rejects_a_non_beacon_frame() {
    let mut f = [0u8; 64];
    let fc = frame_control(TYPE_MGMT, SUBTYPE_PROBE_REQ);
    write_header(&mut f, fc, BROADCAST, BROADCAST, BROADCAST, 0).unwrap();
    assert!(parse_beacon(&f).is_none(), "a probe request is not a beacon");
}
