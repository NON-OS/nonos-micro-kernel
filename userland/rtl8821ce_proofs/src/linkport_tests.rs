// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that the RTL8821CE `RtlLink` satisfies the shared `LinkPort` contract
//! net_core drives. An unassociated link reports down, offers no MAC and refuses
//! to transmit; once associated it reports its MAC and link up. A transmitted
//! Ethernet frame is framed by the station and laid into the TX ring with the
//! hardware CCMP security type set, so the radio encrypts from the CAM, and the
//! write-index doorbell is rung. A received 802.11 MPDU is pulled off the RX ring
//! and parsed back to the exact Ethernet frame that produced it. Driven through
//! the real `nonos_wifi_core::netif::LinkPort` trait against modeled DMA and a
//! modeled card; only the card actually moving bytes is left for on-silicon
//! bring-up.

use std::cell::RefCell;
use std::rc::Rc;

use crate::fw::dma::DmaMem;
use crate::link::{RtlLink, RxBuffers};
use crate::regs::Mmio;
use crate::rx::regs::REG_RXBD_IDX_MPDUQ;
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::tx::desc::TXDESC_LEN;
use crate::tx::regs::{REG_TXBD_IDX_BEQ, TRX_BD_HW_IDX_SHIFT};

use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::station::{Ccmp, LinkStation};

const STA_MAC: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
const AP_MAC: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

// A modeled DMA region: readable and writable bytes with a bus address. The
// backing store is shared so a test can inspect what the driver laid down after
// the link has taken ownership of the region.
#[derive(Clone)]
struct Dma {
    mem: Rc<RefCell<Vec<u8>>>,
    dev: u64,
}

impl Dma {
    fn new(len: usize, dev: u64) -> Self {
        Self { mem: Rc::new(RefCell::new(vec![0u8; len])), dev }
    }
}

impl DmaMem for Dma {
    fn capacity(&self) -> usize {
        self.mem.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.mem.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

// The receive buffers as a CPU-readable region. The poll path only reads them, so
// a plain vector suffices; the harness stages a frame before the link is built.
struct RxBuf {
    mem: Vec<u8>,
    dev: u64,
}

impl RxBuf {
    fn new(dev: u64) -> Self {
        Self { mem: vec![0u8; RX_DESC_COUNT as usize * RX_BUF_STRIDE], dev }
    }

    // Stage a decrypted 802.11 MPDU behind a clean data descriptor in `slot`.
    fn stage(&mut self, slot: u32, mpdu: &[u8]) {
        let base = RxState::buffer_offset(slot);
        self.mem[base..base + 24].copy_from_slice(&rx_desc(mpdu.len() as u16));
        self.mem[base + 24..base + 24 + mpdu.len()].copy_from_slice(mpdu);
    }
}

impl RxBuffers for RxBuf {
    fn bytes(&self) -> &[u8] {
        &self.mem
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
}

// A clean data RX descriptor: length only, no driver info, no shift, no errors,
// not a firmware command. The frame body sits at the fixed 24-byte offset.
fn rx_desc(pkt_len: u16) -> [u8; 24] {
    let w0 = (pkt_len as u32) & 0x3FFF;
    let mut d = [0u8; 24];
    d[0..4].copy_from_slice(&w0.to_le_bytes());
    d
}

// A modeled card. Both ring index registers report a settable hardware pointer in
// their high bits; every write is recorded in a shared log so a test can assert
// the doorbell after the card has moved into the link.
#[derive(Clone)]
struct Card {
    tx_rp: u32,
    rx_wp: u32,
    writes: Rc<RefCell<Vec<(usize, u32)>>>,
}

impl Card {
    fn new(tx_rp: u32, rx_wp: u32) -> Self {
        Self { tx_rp, rx_wp, writes: Rc::new(RefCell::new(Vec::new())) }
    }
    fn wrote(&self, off: usize) -> bool {
        self.writes.borrow().iter().any(|&(o, _)| o == off)
    }
}

impl Mmio for Card {
    fn read8(&self, _o: usize) -> u8 {
        0
    }
    fn write8(&self, _o: usize, _v: u8) {}
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, off: usize, val: u16) {
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read32(&self, off: usize) -> u32 {
        if off == REG_TXBD_IDX_BEQ {
            self.tx_rp << TRX_BD_HW_IDX_SHIFT
        } else if off == REG_RXBD_IDX_MPDUQ {
            self.rx_wp << TRX_BD_HW_IDX_SHIFT
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

// Build a link over fresh DMA. Returns the link, a handle on the TX buffer store
// so a test can read the descriptor the driver emitted, and the card handle.
fn link_with(card: Card, rx: RxBuf) -> (RtlLink<Card, Dma, RxBuf>, Dma) {
    let tx_ring = Dma::new(1 << 12, 0x1000_0000);
    let tx_buffers = Dma::new(1 << 16, 0x2000_0000);
    let rx_ring = Dma::new(1 << 12, 0x3000_0000);
    let tx_handle = tx_buffers.clone();
    let link = RtlLink::new(card, tx_ring, tx_buffers, rx_ring, rx, STA_MAC);
    (link, tx_handle)
}

// A minimal Ethernet frame to `dst` from `src` with one payload byte.
fn eth_frame(dst: [u8; 6], src: [u8; 6], ethertype: [u8; 2], payload: &[u8]) -> Vec<u8> {
    let mut e = Vec::new();
    e.extend_from_slice(&dst);
    e.extend_from_slice(&src);
    e.extend_from_slice(&ethertype);
    e.extend_from_slice(payload);
    e
}

#[test]
fn an_unassociated_link_is_down_with_no_mac() {
    let (link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    assert!(!link.link_up(), "no association means link down");
    assert_eq!(link.mac(), None, "no MAC is offered until associated");
}

#[test]
fn an_unassociated_link_refuses_to_transmit() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    let eth = [0u8; 64];
    assert!(!link.send_tx(&eth), "an unassociated station cannot frame a packet");
}

#[test]
fn association_brings_the_link_up_and_publishes_the_mac() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC);
    assert!(link.link_up(), "associated means link up");
    assert_eq!(link.mac(), Some(STA_MAC), "the station MAC is published once up");
}

#[test]
fn transmit_frames_the_packet_with_hardware_ccmp_and_kicks_the_queue() {
    let card = Card::new(0, 0);
    let probe = card.clone();
    let (mut link, tx) = link_with(card, RxBuf::new(0x4000_0000));
    link.associate(AP_MAC);

    let eth = eth_frame(AP_MAC, STA_MAC, [0x08, 0x00], &[0xAB]);
    assert!(link.send_tx(&eth), "an associated station frames and enqueues");

    // The descriptor sits at buffer offset 0; word 1 carries the security type in
    // bits 22..23. Hardware CCMP is type 3: the radio encrypts from the CAM.
    let mem = tx.mem.borrow();
    let w1 = u32::from_le_bytes([mem[4], mem[5], mem[6], mem[7]]);
    assert_eq!((w1 >> 22) & 0x3, 3, "the TX descriptor requests hardware CCMP");
    // The framed MPDU follows the descriptor: its first bytes are the 802.11
    // frame control, non-zero, so real framing (not a zeroed slot) happened.
    assert!(mem[TXDESC_LEN] != 0, "a framed 802.11 MPDU was laid down");
    drop(mem);

    // The write-index doorbell was rung so the card sees the new descriptor.
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "the BE queue write index is kicked");
}

#[test]
fn receive_recovers_the_original_ethernet_frame() {
    // A peer station frames an Ethernet packet exactly as the AP's radio would
    // hand it up after decryption: a plaintext 802.11 data MPDU.
    let mut peer = LinkStation::new(AP_MAC);
    peer.associate(STA_MAC, Ccmp::Hardware);
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x06], &[0xDE, 0xAD, 0xBE, 0xEF]);
    let mpdu = peer.tx_frame(&eth).expect("the peer frames the packet");

    // Stage it in RX slot 0 and tell the card one frame is ready.
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &mpdu);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC);

    let mut out = [0u8; 1600];
    let n = link.poll_rx(&mut out).expect("a staged frame is delivered");
    assert_eq!(&out[..n], &eth[..], "the exact Ethernet frame is recovered");
}

#[test]
fn receive_on_an_empty_ring_yields_nothing() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "no frame queued means no delivery");
}
