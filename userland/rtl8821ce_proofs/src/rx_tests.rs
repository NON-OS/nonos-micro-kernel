// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer and known-behaviour proofs for the MPDU RX path: the RX
//! descriptor parser (frame length, driver-info and shift offsets, error and
//! command flags), the re-arm buffer descriptor, the ring readiness arithmetic,
//! the ring setup that primes and points the card at the ring, and the poll that
//! lifts one frame, skipping firmware-command, error and oversized frames. A
//! modeled device and DMA memory stand in for the card; only the card actually
//! receiving is left for on-silicon bring-up.

use core::cell::RefCell;

use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::desc::{self, parse};
use crate::rx::regs::{REG_RXBD_DESA_MPDUQ, REG_RXBD_IDX_MPDUQ, REG_RXBD_NUM_MPDUQ};
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::rx::{poll_one, program};
use crate::tx::regs::TRX_BD_HW_IDX_SHIFT;

// The bus address the modeled packet buffer sits at.
const BUF_DEV: u64 = 0xA000_0000;

// The buffer-descriptor ring, host-written and card-read.
struct MockRing {
    mem: RefCell<Vec<u8>>,
    device_addr: u64,
}

impl MockRing {
    fn new(len: usize, device_addr: u64) -> Self {
        Self { mem: RefCell::new(vec![0u8; len]), device_addr }
    }
}

impl DmaMem for MockRing {
    fn capacity(&self) -> usize {
        self.mem.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.device_addr
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.mem.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

// A modeled card: the RX index register reports a settable hardware write index
// in its high bits; writes are recorded.
struct Card {
    hw_wp: u32,
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Card {
    fn new(hw_wp: u32) -> Self {
        Self { hw_wp, writes: RefCell::new(Vec::new()) }
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
        if off == REG_RXBD_IDX_MPDUQ {
            self.hw_wp << TRX_BD_HW_IDX_SHIFT
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

// Build a 24-byte RX descriptor word 0 (and the C2H bit in word 2).
fn rx_desc(pkt_len: u16, drv_units: u8, shift: u8, crc: bool, icv: bool, c2h: bool) -> [u8; 24] {
    let mut w0 = (pkt_len as u32) & 0x3FFF;
    w0 |= ((drv_units as u32) & 0xF) << 16;
    w0 |= ((shift as u32) & 0x3) << 24;
    if crc {
        w0 |= 1 << 14;
    }
    if icv {
        w0 |= 1 << 15;
    }
    let w2 = if c2h { 1u32 << 28 } else { 0 };
    let mut d = [0u8; 24];
    d[0..4].copy_from_slice(&w0.to_le_bytes());
    d[8..12].copy_from_slice(&w2.to_le_bytes());
    d
}

// Place a descriptor and frame body into a buffer slot.
fn place(buffers: &mut [u8], slot: u32, desc_bytes: &[u8], frame: &[u8], frame_off: usize) {
    let base = RxState::buffer_offset(slot);
    buffers[base..base + desc_bytes.len()].copy_from_slice(desc_bytes);
    buffers[base + frame_off..base + frame_off + frame.len()].copy_from_slice(frame);
}

fn new_buffers() -> Vec<u8> {
    vec![0u8; RX_DESC_COUNT as usize * RX_BUF_STRIDE]
}

#[test]
fn descriptor_parse_gives_frame_offset_and_flags() {
    // drv_info 2 units (16 bytes) and a 2-byte shift: frame offset 24+16+2 = 42.
    let d = rx_desc(0x40, 2, 2, false, false, false);
    let info = parse(&d).unwrap();
    assert_eq!(info.pkt_len, 0x40, "frame length");
    assert_eq!(info.drv_info_size, 16, "driver info in bytes");
    assert_eq!(info.shift, 2, "shift");
    assert_eq!(info.frame_offset(), 42, "offset to the frame");
    assert_eq!(info.total_len(), 42 + 0x40, "descriptor plus frame");
    assert!(info.deliverable(), "a clean data frame is deliverable");
}

#[test]
fn error_and_command_frames_are_not_deliverable() {
    assert!(!parse(&rx_desc(64, 0, 0, true, false, false)).unwrap().deliverable(), "crc error");
    assert!(!parse(&rx_desc(64, 0, 0, false, true, false)).unwrap().deliverable(), "icv error");
    assert!(!parse(&rx_desc(64, 0, 0, false, false, true)).unwrap().deliverable(), "c2h command");
    assert!(!parse(&rx_desc(0, 0, 0, false, false, false)).unwrap().deliverable(), "empty frame");
    assert!(parse(&[0u8; 8]).is_none(), "a runt buffer has no descriptor");
}

#[test]
fn rearm_descriptor_carries_size_and_address() {
    let bd = desc::rearm(0x8000_4000, RX_BUF_STRIDE);
    assert_eq!(u16::from_le_bytes([bd[0], bd[1]]), RX_BUF_STRIDE as u16, "buffer size");
    assert_eq!(u16::from_le_bytes([bd[2], bd[3]]), 0, "received length starts zero");
    assert_eq!(u32::from_le_bytes([bd[4], bd[5], bd[6], bd[7]]), 0x8000_4000, "buffer address");
}

#[test]
fn ready_counts_the_gap_up_to_the_hardware_index() {
    let mut s = RxState::new(RX_DESC_COUNT);
    assert_eq!(s.ready(0), 0, "no frames when the indices meet");
    assert_eq!(s.ready(3), 3, "three frames ready");
    s.rp = RX_DESC_COUNT - 1;
    assert_eq!(s.ready(1), 2, "the count wraps around the ring end");
}

#[test]
fn setup_primes_descriptors_and_points_the_card_at_the_ring() {
    let card = Card::new(0);
    let ring = MockRing::new(RX_DESC_COUNT as usize * 8, 0x9000_0000);
    program(&card, &ring, BUF_DEV);

    // Slot 3's descriptor points at slot 3's buffer.
    let bd = ring.mem.borrow()[24..32].to_vec();
    let want = desc::rearm(BUF_DEV + RxState::buffer_offset(3) as u64, RX_BUF_STRIDE);
    assert_eq!(bd, want, "descriptor 3 armed against buffer 3");
    let w = card.writes.borrow();
    assert!(w.contains(&(REG_RXBD_DESA_MPDUQ, 0x9000_0000)), "ring address low");
    assert!(w.contains(&(REG_RXBD_NUM_MPDUQ, RX_DESC_COUNT)), "ring length");
}

#[test]
fn poll_lifts_a_frame_and_rearms_the_slot() {
    let card = Card::new(1); // one frame ready at slot 0
    let ring = MockRing::new(RX_DESC_COUNT as usize * 8, 0x9000_0000);
    let mut buffers = new_buffers();
    let mut state = RxState::new(RX_DESC_COUNT);

    let frame = [0x77u8; 100];
    place(&mut buffers, 0, &rx_desc(frame.len() as u16, 0, 0, false, false, false), &frame, 24);

    let mut out = [0u8; 1600];
    let n = poll_one(&card, &ring, &buffers, BUF_DEV, &mut state, &mut out)
        .expect("a frame is delivered");
    assert_eq!(n, frame.len(), "the frame length is reported");
    assert_eq!(&out[..n], &frame[..], "the frame body is copied out");
    assert_eq!(state.rp, 1, "the read index advanced");
    assert!(card.writes.borrow().contains(&(REG_RXBD_IDX_MPDUQ, 1)), "the slot is handed back");
    // The consumed slot was re-armed against its buffer.
    let want = desc::rearm(BUF_DEV, RX_BUF_STRIDE);
    assert_eq!(&ring.mem.borrow()[..8], &want[..], "slot 0 re-armed");
}

#[test]
fn poll_skips_error_frames_and_returns_the_next_good_one() {
    let card = Card::new(2); // two frames ready: a crc error then a good one
    let ring = MockRing::new(RX_DESC_COUNT as usize * 8, 0x9000_0000);
    let mut buffers = new_buffers();
    let mut state = RxState::new(RX_DESC_COUNT);

    place(&mut buffers, 0, &rx_desc(50, 0, 0, true, false, false), &[0xEE; 50], 24);
    let good = [0x22u8; 80];
    place(&mut buffers, 1, &rx_desc(good.len() as u16, 0, 0, false, false, false), &good, 24);

    let mut out = [0u8; 1600];
    let n =
        poll_one(&card, &ring, &buffers, BUF_DEV, &mut state, &mut out).expect("the good frame");
    assert_eq!(&out[..n], &good[..], "the error frame is skipped");
    assert_eq!(state.rp, 2, "both slots consumed");
}

#[test]
fn poll_returns_none_when_no_frame_is_ready() {
    let card = Card::new(0);
    let ring = MockRing::new(RX_DESC_COUNT as usize * 8, 0x9000_0000);
    let buffers = new_buffers();
    let mut state = RxState::new(RX_DESC_COUNT);
    let mut out = [0u8; 1600];
    assert!(
        poll_one(&card, &ring, &buffers, BUF_DEV, &mut state, &mut out).is_none(),
        "nothing ready"
    );
}
