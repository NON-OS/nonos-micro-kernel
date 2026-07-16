// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the WiFi panel state machine: the detect -> select -> enter-key
//! -> connect flow. A scan populates the list deduped and ordered by signal;
//! the cursor stays in bounds; choosing a secured network opens the passphrase
//! editor and an open one does not; the passphrase honours the WPA2 length
//! bounds; and a connect request is produced only when the selection and
//! passphrase are valid.

use crate::wifi::panel::{WifiPanel, WifiStatus};

// Build a scan buffer: [count] then [signal][flags][ssid_len][ssid] per entry.
fn scan(entries: &[(u8, bool, &[u8])]) -> Vec<u8> {
    let mut buf = vec![entries.len() as u8];
    for &(signal, secured, ssid) in entries {
        buf.push(signal);
        buf.push(if secured { 1 } else { 0 });
        buf.push(ssid.len() as u8);
        buf.extend_from_slice(ssid);
    }
    buf
}

#[test]
fn a_fresh_panel_is_idle_and_empty() {
    let p = WifiPanel::default();
    assert_eq!(p.status(), WifiStatus::Idle);
    assert!(p.networks().is_empty());
}

#[test]
fn begin_scan_moves_to_scanning() {
    let mut p = WifiPanel::default();
    p.begin_scan();
    assert_eq!(p.status(), WifiStatus::Scanning);
}

#[test]
fn load_scan_orders_networks_strongest_first() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(80, true, b"weak"), (240, true, b"strong"), (150, false, b"mid")]));
    let names: Vec<&[u8]> = p.networks().iter().map(|n| n.ssid()).collect();
    assert_eq!(names, vec![&b"strong"[..], &b"mid"[..], &b"weak"[..]], "sorted by signal");
    assert_eq!(p.status(), WifiStatus::Idle);
}

#[test]
fn load_scan_merges_duplicate_ssids_keeping_the_strongest() {
    let mut p = WifiPanel::default();
    // Same SSID seen twice (two APs); the stronger signal wins, listed once.
    p.load_scan(&scan(&[(90, true, b"home"), (200, true, b"home"), (60, false, b"other")]));
    assert_eq!(p.networks().len(), 2, "the duplicate SSID is merged");
    assert_eq!(p.networks()[0].ssid(), b"home");
    assert_eq!(p.networks()[0].signal, 200, "the stronger signal is kept");
}

#[test]
fn the_cursor_stays_within_the_list() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, false, b"a"), (100, false, b"b")]));
    p.move_up(); // already at 0
    assert_eq!(p.cursor(), 0);
    p.move_down();
    assert_eq!(p.cursor(), 1);
    p.move_down(); // clamped at the last row
    assert_eq!(p.cursor(), 1);
}

#[test]
fn choosing_a_secured_network_opens_the_passphrase_editor() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, true, b"secured")]));
    p.choose();
    assert_eq!(p.status(), WifiStatus::Entering);
}

#[test]
fn choosing_an_open_network_does_not_open_the_editor() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, false, b"open")]));
    p.choose();
    assert_ne!(p.status(), WifiStatus::Entering);
    assert!(p.passphrase_valid(), "an open network needs no passphrase");
}

#[test]
fn passphrase_editing_respects_the_wpa2_bounds() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, true, b"secured")]));
    p.choose();
    // Too short: not valid until 8 characters.
    for _ in 0..7 {
        p.push_pass(b'a');
    }
    assert!(!p.passphrase_valid(), "seven characters is too short");
    p.push_pass(b'a');
    assert!(p.passphrase_valid(), "eight characters is valid");
    // Cannot exceed 63 characters.
    for _ in 0..100 {
        p.push_pass(b'x');
    }
    assert_eq!(p.passphrase_len(), 63, "the passphrase is capped at 63");
    assert!(p.passphrase_valid());
    p.backspace();
    assert_eq!(p.passphrase_len(), 62);
}

#[test]
fn a_secured_network_will_not_connect_without_a_valid_passphrase() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, true, b"secured")]));
    p.choose();
    p.push_pass(b'a');
    p.push_pass(b'b'); // only 2 chars
    assert!(p.connect_request().is_none(), "a short passphrase is refused");
    assert_ne!(p.status(), WifiStatus::Connecting, "no connect on refusal");
}

#[test]
fn a_valid_secured_connect_produces_a_request_and_moves_to_connecting() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, true, b"home")]));
    p.choose();
    for c in b"secret12" {
        p.push_pass(*c);
    }
    let req = p.connect_request().expect("a valid request");
    assert_eq!(req[0], 4, "ssid length");
    assert_eq!(&req[1..5], b"home");
    assert_eq!(&req[5..], b"secret12");
    assert_eq!(p.status(), WifiStatus::Connecting);
}

#[test]
fn an_open_network_connects_with_no_passphrase() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, false, b"guest")]));
    p.choose();
    let req = p.connect_request().expect("open networks connect immediately");
    assert_eq!(&req[1..], b"guest");
    assert_eq!(p.status(), WifiStatus::Connecting);
}

#[test]
fn the_service_outcome_is_recorded() {
    let mut p = WifiPanel::default();
    p.load_scan(&scan(&[(200, false, b"guest")]));
    p.choose();
    let _ = p.connect_request();
    p.set_connected(true);
    assert_eq!(p.status(), WifiStatus::Connected);
    p.set_connected(false);
    assert_eq!(p.status(), WifiStatus::Failed);
}
