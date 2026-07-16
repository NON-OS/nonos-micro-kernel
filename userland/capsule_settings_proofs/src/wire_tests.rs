// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the WiFi scan/connect wire format, which parses UNTRUSTED bytes
//! from the WiFi service. A well-formed buffer yields the right networks; every
//! malformed shape (a lying count, a truncated entry, an SSID length running
//! past the buffer or over the maximum) is refused at the first bad entry
//! without panicking or reading out of bounds. The connect encoder round-trips
//! and clamps an oversized SSID.

use crate::wifi::network::{ScanNetwork, SSID_MAX};
use crate::wifi::wire::{encode_connect, parse_scan};

fn collect(buf: &[u8]) -> Vec<ScanNetwork> {
    let mut out = Vec::new();
    parse_scan(buf, |n| out.push(n));
    out
}

// [count] then [signal][flags][ssid_len][ssid] per entry.
fn entry(signal: u8, secured: bool, ssid: &[u8]) -> Vec<u8> {
    let mut e = vec![signal, if secured { 1 } else { 0 }, ssid.len() as u8];
    e.extend_from_slice(ssid);
    e
}

#[test]
fn a_well_formed_buffer_parses_every_network() {
    let mut buf = vec![2u8];
    buf.extend(entry(200, true, b"home"));
    buf.extend(entry(120, false, b"cafe"));
    let nets = collect(&buf);
    assert_eq!(nets.len(), 2);
    assert_eq!(nets[0].ssid(), b"home");
    assert_eq!(nets[0].signal, 200);
    assert!(nets[0].secured, "flags bit 0 marks secured");
    assert_eq!(nets[1].ssid(), b"cafe");
    assert!(!nets[1].secured);
}

#[test]
fn an_empty_buffer_yields_nothing() {
    assert_eq!(collect(&[]).len(), 0);
    assert_eq!(collect(&[0]).len(), 0, "a count of zero");
}

#[test]
fn a_lying_count_stops_at_the_real_data() {
    // Count claims 5 but only one entry follows.
    let mut buf = vec![5u8];
    buf.extend(entry(90, false, b"one"));
    assert_eq!(collect(&buf).len(), 1, "parsing stops when the data runs out");
}

#[test]
fn a_truncated_entry_header_is_refused() {
    // Count says 1, but only two of the three header bytes are present.
    assert_eq!(collect(&[1, 50, 0]).len(), 0, "a partial entry header is dropped");
}

#[test]
fn an_ssid_running_past_the_buffer_is_refused() {
    // ssid_len = 10 but only 3 bytes follow.
    let buf = vec![1u8, 100, 0, 10, b'a', b'b', b'c'];
    assert_eq!(collect(&buf).len(), 0, "an SSID past the end is not read");
}

#[test]
fn an_oversized_ssid_length_is_refused() {
    // ssid_len = 40 exceeds SSID_MAX (32); refuse rather than truncate silently.
    let mut buf = vec![1u8, 100, 0, 40];
    buf.extend([b'x'; 40]);
    assert_eq!(collect(&buf).len(), 0, "an over-long SSID is rejected");
}

#[test]
fn a_maximum_length_ssid_is_accepted() {
    let ssid = [b'z'; SSID_MAX];
    let mut buf = vec![1u8];
    buf.extend(entry(255, true, &ssid));
    let nets = collect(&buf);
    assert_eq!(nets.len(), 1);
    assert_eq!(nets[0].ssid(), &ssid[..]);
}

#[test]
fn connect_request_encodes_ssid_length_ssid_then_passphrase() {
    let req = encode_connect(b"home", b"secret12");
    assert_eq!(req[0], 4, "ssid length");
    assert_eq!(&req[1..5], b"home");
    assert_eq!(&req[5..], b"secret12", "passphrase follows");
}

#[test]
fn connect_request_for_an_open_network_has_no_passphrase() {
    let req = encode_connect(b"guest", b"");
    assert_eq!(req[0], 5);
    assert_eq!(&req[1..], b"guest", "no passphrase bytes for an open network");
}

#[test]
fn connect_request_clamps_an_oversized_ssid() {
    let ssid = [b'a'; 40];
    let req = encode_connect(&ssid, b"pw");
    assert_eq!(req[0] as usize, SSID_MAX, "the SSID length is clamped to the maximum");
    assert_eq!(req.len(), 1 + SSID_MAX + 2, "only the clamped SSID and the passphrase are sent");
}
