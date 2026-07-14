// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proof for the station association MLME. A scripted access point
//! drives the real `Mlme` through a full join: it answers the beacon with the
//! MLME's authentication and association requests, then runs the WPA2 four-way
//! handshake, and the proof asserts the MLME reaches Connected with the pairwise
//! and group keys, that each stage emits the right frame, and that a refused
//! authentication and a beacon for the wrong SSID are handled without joining.
//! No hardware: every frame is built and parsed on the host.

use crate::ccmp::aes::Aes128;
use crate::dot11::header::{
    frame_control, MAC_HEADER_LEN, SUBTYPE_ASSOC_RESP, SUBTYPE_AUTH, SUBTYPE_BEACON, TYPE_MGMT,
};
use crate::eapol::build::build_key_frame;
use crate::eapol::parse::{KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_VERSION2};
use crate::mlme::{Mlme, MlmeState};
use crate::wpa::ptk::{pmk, ptk};

const OUR_MAC: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
const AP_MAC: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const SSID: &[u8] = b"ThisIsASSID";
const PASSPHRASE: &[u8] = b"ThisIsAPassword";
const SNONCE: [u8; 32] = [0x52; 32];
const ANONCE: [u8; 32] = [0xA1; 32];
const GTK: [u8; 16] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
];
const CHANNEL: u8 = 6;

fn mgmt_header(subtype: u8) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    f[0..2].copy_from_slice(&frame_control(TYPE_MGMT, subtype).to_le_bytes());
    f[4..10].copy_from_slice(&[0xff; 6]); // addr1 (to us / broadcast)
    f[10..16].copy_from_slice(&AP_MAC); // addr2 (AP)
    f[16..22].copy_from_slice(&AP_MAC); // addr3 (BSSID)
    f
}

fn beacon(ssid: &[u8]) -> Vec<u8> {
    let mut f = mgmt_header(SUBTYPE_BEACON);
    f.extend_from_slice(&[0u8; 8]); // timestamp
    f.extend_from_slice(&100u16.to_le_bytes()); // beacon interval
    f.extend_from_slice(&0x0011u16.to_le_bytes()); // capability: ESS + privacy
    f.extend_from_slice(&[0, ssid.len() as u8]); // SSID IE
    f.extend_from_slice(ssid);
    f.extend_from_slice(&[3, 1, CHANNEL]); // DS parameter set: channel
    f.extend_from_slice(&[48, 2, 0x01, 0x00]); // a (minimal) RSN element
    f
}

fn auth_response(status: u16) -> Vec<u8> {
    let mut f = mgmt_header(SUBTYPE_AUTH);
    f.extend_from_slice(&0u16.to_le_bytes()); // algorithm: open
    f.extend_from_slice(&2u16.to_le_bytes()); // transaction sequence
    f.extend_from_slice(&status.to_le_bytes());
    f
}

fn assoc_response(status: u16) -> Vec<u8> {
    let mut f = mgmt_header(SUBTYPE_ASSOC_RESP);
    f.extend_from_slice(&0x0011u16.to_le_bytes()); // capability
    f.extend_from_slice(&status.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes()); // AID
    f
}

// RFC 3394 AES key wrap, the AP's group-key delivery.
fn aes_wrap(kek: &[u8; 16], plain: &[u8]) -> Vec<u8> {
    let n = plain.len() / 8;
    let aes = Aes128::new(kek);
    let mut a = [0xA6u8; 8];
    let mut r: Vec<[u8; 8]> = (0..n)
        .map(|i| {
            let mut b = [0u8; 8];
            b.copy_from_slice(&plain[i * 8..i * 8 + 8]);
            b
        })
        .collect();
    for j in 0..6u64 {
        for i in 0..n {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&r[i]);
            aes.encrypt_block(&mut block);
            let t = (n as u64) * j + (i as u64) + 1;
            a.copy_from_slice(&block[..8]);
            for k in 0..8 {
                a[k] ^= (t >> (56 - 8 * k)) as u8;
            }
            r[i].copy_from_slice(&block[8..]);
        }
    }
    let mut out = Vec::new();
    out.extend_from_slice(&a);
    for block in &r {
        out.extend_from_slice(block);
    }
    out
}

fn ap_ptk() -> [u8; 48] {
    let k = pmk(PASSPHRASE, SSID);
    // The AP's authenticator/supplicant addresses are the BSSID and the STA.
    ptk(&k, &AP_MAC, &OUR_MAC, &ANONCE, &SNONCE)
}

fn message1() -> Vec<u8> {
    let mut out = [0u8; 160];
    let n = build_key_frame(
        &mut out,
        KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE,
        &[0, 0, 0, 0, 0, 0, 0, 1],
        &ANONCE,
        &[],
        &[0u8; 16],
    )
    .unwrap();
    out[..n].to_vec()
}

fn message3() -> Vec<u8> {
    let p = ap_ptk();
    let mut kde = vec![0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 0x01, 0x00];
    kde.extend_from_slice(&GTK);
    let kek: [u8; 16] = p[16..32].try_into().unwrap();
    let wrapped = aes_wrap(&kek, &kde);
    let mut out = [0u8; 160];
    let n = build_key_frame(
        &mut out,
        KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC,
        &[0, 0, 0, 0, 0, 0, 0, 2],
        &ANONCE,
        &wrapped,
        &p[0..16],
    )
    .unwrap();
    out[..n].to_vec()
}

#[test]
fn full_join_reaches_connected_with_keys() {
    let mut mlme = Mlme::new(OUR_MAC, SSID, PASSPHRASE, SNONCE);

    // Beacon -> authentication request.
    let o = mlme.on_mgmt(&beacon(SSID));
    assert_eq!(mlme.state(), MlmeState::Authenticating);
    assert!(o.tx.is_some(), "an authentication frame is sent");
    assert_eq!(mlme.channel(), CHANNEL, "the BSS channel was learned from the beacon");

    // Auth response -> association request.
    let o = mlme.on_mgmt(&auth_response(0));
    assert_eq!(mlme.state(), MlmeState::Associating);
    assert!(o.tx.is_some(), "an association frame is sent");

    // Assoc response -> four-way handshake begins.
    let o = mlme.on_mgmt(&assoc_response(0));
    assert_eq!(mlme.state(), MlmeState::FourWay);
    assert!(o.tx.is_none(), "the AP sends message 1 next");

    // Message 1 -> message 2.
    let o = mlme.on_eapol(&message1());
    assert!(o.tx.is_some(), "message 2 is sent");

    // Message 3 -> connected, keys installed.
    let o = mlme.on_eapol(&message3());
    assert_eq!(mlme.state(), MlmeState::Connected);
    assert!(o.tx.is_some(), "message 4 is sent");
    let p = ap_ptk();
    assert_eq!(mlme.tk(), Some(&p[32..48]), "the installed TK matches the AP's");
    assert_eq!(mlme.gtk(), Some(&GTK[..]), "the group key was installed");
}

#[test]
fn refused_authentication_does_not_join() {
    let mut mlme = Mlme::new(OUR_MAC, SSID, PASSPHRASE, SNONCE);
    mlme.on_mgmt(&beacon(SSID));
    let o = mlme.on_mgmt(&auth_response(1)); // status 1 = failure
    assert_eq!(mlme.state(), MlmeState::Failed);
    assert!(o.tx.is_none());
}

#[test]
fn beacon_for_a_different_ssid_is_ignored() {
    let mut mlme = Mlme::new(OUR_MAC, SSID, PASSPHRASE, SNONCE);
    let o = mlme.on_mgmt(&beacon(b"OtherNetwork"));
    assert_eq!(mlme.state(), MlmeState::Scanning, "keep scanning for the target");
    assert!(o.tx.is_none());
}
