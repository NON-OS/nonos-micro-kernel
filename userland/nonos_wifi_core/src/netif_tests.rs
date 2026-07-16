// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the net_core link adapter: given a net_core request, the WiFi
//! driver produces a response in exactly the shape net_core parses. The four
//! operations, plus the "link down" and "no frame" cases, are checked against a
//! modeled link so the binding is correct without a radio. The frame handed to
//! the link on transmit and returned from it on receive are the Ethernet frames
//! net_core exchanges; the 802.11/CCMP translation is the LinkPort's job.

use crate::netif::serve::{serve, LinkPort, MAX_RESPONSE};
use crate::netif::wire::{
    HDR_LEN, MAGIC_NNET, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_TX_PACKET, VERSION,
};
use core::cell::RefCell;

struct MockLink {
    mac: Option<[u8; 6]>,
    up: bool,
    rx: RefCell<Vec<Vec<u8>>>,
    sent: RefCell<Vec<Vec<u8>>>,
}

impl MockLink {
    fn up_with(mac: [u8; 6]) -> Self {
        Self { mac: Some(mac), up: true, rx: RefCell::new(Vec::new()), sent: RefCell::new(Vec::new()) }
    }
    fn down() -> Self {
        Self { mac: None, up: false, rx: RefCell::new(Vec::new()), sent: RefCell::new(Vec::new()) }
    }
}

impl LinkPort for MockLink {
    fn mac(&self) -> Option<[u8; 6]> {
        self.mac
    }
    fn link_up(&self) -> bool {
        self.up
    }
    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        let mut rx = self.rx.borrow_mut();
        if rx.is_empty() {
            return None;
        }
        let f = rx.remove(0);
        out[..f.len()].copy_from_slice(&f);
        Some(f.len())
    }
    fn send_tx(&mut self, frame: &[u8]) -> bool {
        if !self.up {
            return false;
        }
        self.sent.borrow_mut().push(frame.to_vec());
        true
    }
}

fn request(op: u16, rid: u32, payload: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; HDR_LEN + payload.len()];
    v[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    v[4..6].copy_from_slice(&VERSION.to_le_bytes());
    v[6..8].copy_from_slice(&op.to_le_bytes());
    v[12..16].copy_from_slice(&rid.to_le_bytes());
    v[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    v[HDR_LEN..].copy_from_slice(payload);
    v
}

// Parse a response the way net_core does: verify the envelope and return
// (op, request_id, payload_len).
fn parse(resp: &[u8]) -> (u16, u32, u32) {
    assert_eq!(u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]), MAGIC_NNET);
    assert_eq!(u16::from_le_bytes([resp[4], resp[5]]), VERSION);
    let op = u16::from_le_bytes([resp[6], resp[7]]);
    let rid = u32::from_le_bytes([resp[12], resp[13], resp[14], resp[15]]);
    let plen = u32::from_le_bytes([resp[16], resp[17], resp[18], resp[19]]);
    (op, rid, plen)
}

fn status(resp: &[u8]) -> i32 {
    i32::from_le_bytes([resp[HDR_LEN], resp[HDR_LEN + 1], resp[HDR_LEN + 2], resp[HDR_LEN + 3]])
}

#[test]
fn mac_address_reports_the_station_mac_when_associated() {
    let mac = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
    let mut link = MockLink::up_with(mac);
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_MAC_ADDRESS, 7, &[]), &mut link, &mut out).unwrap();
    let (op, rid, plen) = parse(&out[..n]);
    assert_eq!((op, rid, plen), (OP_MAC_ADDRESS, 7, 10));
    assert_eq!(status(&out[..n]), 0);
    assert_eq!(&out[HDR_LEN + 4..HDR_LEN + 10], &mac);
}

#[test]
fn mac_address_is_a_no_link_error_when_unassociated() {
    let mut link = MockLink::down();
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_MAC_ADDRESS, 1, &[]), &mut link, &mut out).unwrap();
    let (op, _, plen) = parse(&out[..n]);
    assert_eq!((op, plen), (OP_MAC_ADDRESS, 10));
    assert_ne!(status(&out[..n]), 0, "no MAC yet reads as an error to net_core");
}

#[test]
fn link_status_reports_up_and_down() {
    let mut up = MockLink::up_with([0; 6]);
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_LINK_STATUS, 2, &[]), &mut up, &mut out).unwrap();
    let (op, _, plen) = parse(&out[..n]);
    assert_eq!((op, plen), (OP_LINK_STATUS, 5));
    assert_eq!(status(&out[..n]), 0);
    assert_eq!(out[HDR_LEN + 4], 1, "associated link reports up");

    let mut down = MockLink::down();
    serve(&request(OP_LINK_STATUS, 3, &[]), &mut down, &mut out).unwrap();
    assert_eq!(out[HDR_LEN + 4], 0, "unassociated link reports down");
}

#[test]
fn transmit_hands_the_ethernet_frame_to_the_link() {
    let mut link = MockLink::up_with([0; 6]);
    let frame = [0xFFu8; 64]; // a broadcast-ish Ethernet frame
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_TX_PACKET, 9, &frame), &mut link, &mut out).unwrap();
    let (op, rid, plen) = parse(&out[..n]);
    assert_eq!((op, rid, plen), (OP_TX_PACKET, 9, 4));
    assert!(status(&out[..n]) >= 0, "accepted");
    assert_eq!(link.sent.borrow().len(), 1);
    assert_eq!(link.sent.borrow()[0], frame.to_vec(), "the frame reaches the CCMP/tx path");
}

#[test]
fn transmit_while_unassociated_is_refused() {
    let mut link = MockLink::down();
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_TX_PACKET, 4, &[0u8; 60]), &mut link, &mut out).unwrap();
    assert!(status(&out[..n]) < 0, "net_core sees the transmit fail");
    assert!(link.sent.borrow().is_empty());
}

#[test]
fn receive_returns_a_queued_ethernet_frame_then_reports_empty() {
    let mut link = MockLink::up_with([0; 6]);
    let frame = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 1, 2, 3, 4, 5, 6, 0x08, 0x00, 0x45];
    link.rx.borrow_mut().push(frame.clone());
    let mut out = [0u8; MAX_RESPONSE];

    let n = serve(&request(OP_RX_PACKET, 5, &[]), &mut link, &mut out).unwrap();
    let (op, rid, plen) = parse(&out[..n]);
    assert_eq!((op, rid), (OP_RX_PACKET, 5));
    assert_eq!(plen as usize, 4 + 4 + frame.len());
    assert_eq!(status(&out[..n]), 0);
    let flen = u32::from_le_bytes([
        out[HDR_LEN + 4],
        out[HDR_LEN + 5],
        out[HDR_LEN + 6],
        out[HDR_LEN + 7],
    ]) as usize;
    assert_eq!(flen, frame.len());
    assert_eq!(&out[HDR_LEN + 8..HDR_LEN + 8 + flen], &frame[..]);

    // The queue is now empty: the next poll is a retry status, no frame.
    let n = serve(&request(OP_RX_PACKET, 6, &[]), &mut link, &mut out).unwrap();
    let (_, _, plen) = parse(&out[..n]);
    assert_eq!(plen, 4);
    assert_ne!(status(&out[..n]), 0, "empty poll is a retry, not a zero-length frame");
}

#[test]
fn a_malformed_request_is_rejected() {
    let mut link = MockLink::up_with([0; 6]);
    let mut out = [0u8; MAX_RESPONSE];
    // Wrong magic.
    let mut bad = request(OP_LINK_STATUS, 1, &[]);
    bad[0] ^= 0xFF;
    assert!(serve(&bad, &mut link, &mut out).is_none());
    // Truncated header.
    assert!(serve(&[0u8; 8], &mut link, &mut out).is_none());
}
