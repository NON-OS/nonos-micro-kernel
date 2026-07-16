// NONOS Operating System (AGPL-3.0-or-later)
//! Known-behaviour proof for the WiFi settings panel: a scan buffer is parsed
//! into a selectable list, the cursor moves within bounds, choosing a secured
//! network opens the passphrase editor and gates connect until a key is
//! entered, an open network connects with no key, and the encoded connect
//! request carries exactly the chosen SSID and passphrase. Malformed scan
//! buffers are read as far as they are valid and no further.

use crate::wifi::{parse_scan, WifiPanel, WifiStatus};

// Build a scan buffer: count, then [signal][flags][ssid_len][ssid] per net.
fn scan(nets: &[(&[u8], u8, bool)]) -> Vec<u8> {
    let mut b = vec![nets.len() as u8];
    for (ssid, signal, secured) in nets {
        b.push(*signal);
        b.push(if *secured { 1 } else { 0 });
        b.push(ssid.len() as u8);
        b.extend_from_slice(ssid);
    }
    b
}

#[test]
fn a_scan_buffer_becomes_a_selectable_list() {
    let mut p = WifiPanel::default();
    p.begin_scan();
    assert_eq!(p.status(), WifiStatus::Scanning);
    p.load_scan(&scan(&[(b"HomeNet", 80, true), (b"Cafe", 55, false)]));
    assert_eq!(p.networks().len(), 2);
    assert_eq!(p.networks()[0].ssid(), b"HomeNet");
    assert!(p.networks()[0].secured);
    assert_eq!(p.networks()[1].ssid(), b"Cafe");
    assert!(!p.networks()[1].secured);
    assert_eq!(p.status(), WifiStatus::Idle);
}

#[test]
fn the_cursor_stays_within_the_list() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(b"A", 10, false), (b"B", 20, false)]));
    assert_eq!(p.cursor(), 0);
    p.move_up(); // already at top
    assert_eq!(p.cursor(), 0);
    p.move_down();
    assert_eq!(p.cursor(), 1);
    p.move_down(); // already at bottom
    assert_eq!(p.cursor(), 1);
}

#[test]
fn a_secured_network_needs_a_passphrase_before_connecting() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(b"Secure", 90, true)]));
    p.choose();
    assert_eq!(p.status(), WifiStatus::Entering, "a secured network opens the editor");
    assert!(p.connect_request().is_none(), "no connect without a passphrase");
    for c in b"hunter2" {
        p.push_pass(*c);
    }
    assert_eq!(p.passphrase_len(), 7);
    p.backspace();
    assert_eq!(p.passphrase_len(), 6);
    let req = p.connect_request().expect("connect once a key is entered");
    // [ssid_len][ssid][passphrase]
    assert_eq!(req[0] as usize, 6, "SSID length");
    assert_eq!(&req[1..7], b"Secure");
    assert_eq!(&req[7..], b"hunter", "the entered passphrase, minus the backspaced char");
    assert_eq!(p.status(), WifiStatus::Connecting);
}

#[test]
fn an_open_network_connects_with_no_key() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(b"OpenAir", 40, false)]));
    p.choose();
    assert_eq!(p.status(), WifiStatus::Idle, "an open network needs no editor");
    let req = p.connect_request().expect("connect straight away");
    assert_eq!(req[0] as usize, 7);
    assert_eq!(&req[1..8], b"OpenAir");
    assert_eq!(req.len(), 8, "no passphrase bytes");
}

#[test]
fn the_service_result_sets_the_status() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(b"N", 50, false)]));
    p.choose();
    p.connect_request();
    p.set_connected(true);
    assert_eq!(p.status(), WifiStatus::Connected);
    p.set_connected(false);
    assert_eq!(p.status(), WifiStatus::Failed);
}

#[test]
fn a_truncated_scan_is_read_only_as_far_as_valid() {
    // Claims two networks but the second's SSID runs past the buffer.
    let mut buf = scan(&[(b"First", 60, false)]);
    buf[0] = 2; // lie about the count
    buf.extend_from_slice(&[30, 0, 40]); // signal, flags, ssid_len=40 with no ssid
    let mut n = 0;
    parse_scan(&buf, |_| n += 1);
    assert_eq!(n, 1, "only the well-formed network is read");
}
