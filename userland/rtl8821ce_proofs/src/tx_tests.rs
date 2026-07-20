// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer and known-behaviour proofs for the BE data TX path: the TX
//! packet descriptor fields, the ring index arithmetic (avail with one reserved
//! descriptor, wrap, hardware read-index adoption), the ring register setup, and
//! the enqueue that lays a frame into its slot, publishes the buffer descriptor,
//! advances the write index and kicks the queue. A modeled device and DMA memory
//! stand in for the card; only the card actually transmitting is left for
//! on-silicon bring-up.

use core::cell::RefCell;

use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::TX_DESC_SIZE;
use crate::tx::desc::{self, FrameMeta, SEC_TYPE_CCMP};

/// `DESC_RATE1M`: the slowest, most robust rate code, the one a management frame
/// is sent at. Defined here as the expected descriptor value; the driver's own
/// management-frame path installs it when association is wired.
const DESC_RATE_1M: u8 = 0x00;
use crate::tx::regs::{
    QSEL_BE, REG_TXBD_DESA_BEQ, REG_TXBD_IDX_BEQ, REG_TXBD_NUM_BEQ, TRX_BD_HW_IDX_SHIFT,
};
use crate::tx::ring::{TxState, TX_BUF_STRIDE, TX_DESC_COUNT};
use crate::tx::{enqueue, program};

struct MockDma {
    mem: RefCell<Vec<u8>>,
    device_addr: u64,
}

impl MockDma {
    fn new(len: usize, device_addr: u64) -> Self {
        Self { mem: RefCell::new(vec![0u8; len]), device_addr }
    }
}

impl DmaMem for MockDma {
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

// A modeled card: records writes and returns a settable hardware read index in
// the BE index register.
struct Card {
    hw_rp: u32,
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Card {
    fn new(hw_rp: u32) -> Self {
        Self { hw_rp, writes: RefCell::new(Vec::new()) }
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
            self.hw_rp << TRX_BD_HW_IDX_SHIFT
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn a_firmware_rate_data_descriptor_has_the_expected_fields() {
    let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 0x123, sec_type: 0 };
    let d = desc::frame(0x5DC, &meta); // a 1500-byte frame
    let w0 = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
    let w1 = u32::from_le_bytes([d[4], d[5], d[6], d[7]]);
    let w3 = u32::from_le_bytes([d[12], d[13], d[14], d[15]]);
    let w4 = u32::from_le_bytes([d[16], d[17], d[18], d[19]]);
    let w9 = u32::from_le_bytes([d[36], d[37], d[38], d[39]]);
    assert_eq!(w0 & 0xFFFF, 0x5DC, "frame size in word 0");
    assert_eq!((w0 >> 16) & 0xFF, TX_DESC_SIZE, "48-byte descriptor offset");
    assert_ne!(w0 & (1 << 26), 0, "last segment set");
    assert_eq!(w0 & (1 << 24), 0, "unicast: no BMC");
    assert_eq!((w1 >> 8) & 0x1F, QSEL_BE as u32, "BE queue selector");
    assert_eq!((w1 >> 16) & 0x1F, 6, "rate id 6");
    assert_eq!(w3 & (1 << 8), 0, "firmware rate control: use_rate clear");
    assert_eq!(w4 & 0x7F, 0x04, "DESC_RATE6M hint");
    assert_eq!((w9 >> 12) & 0xFFF, 0x123, "sequence number");
}

#[test]
fn a_fixed_rate_management_descriptor_sets_use_rate() {
    let meta = FrameMeta { qsel: 18, bmc: true, rate: Some(DESC_RATE_1M), seq: 0, sec_type: 0 };
    let d = desc::frame(64, &meta);
    let w0 = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
    let w3 = u32::from_le_bytes([d[12], d[13], d[14], d[15]]);
    let w4 = u32::from_le_bytes([d[16], d[17], d[18], d[19]]);
    assert_ne!(w0 & (1 << 24), 0, "broadcast: BMC set");
    assert_ne!(w3 & (1 << 8), 0, "fixed rate: use_rate set");
    assert_ne!(w3 & (1 << 10), 0, "fixed rate: rate fallback disabled");
    assert_eq!(w4 & 0x7F, 0x00, "DESC_RATE1M");
}

#[test]
fn a_ccmp_frame_sets_the_hardware_security_type() {
    // With a CCMP key installed, the descriptor selects hardware AES so the card
    // encrypts using the CAM key for the receiver.
    let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 0, sec_type: SEC_TYPE_CCMP };
    let d = desc::frame(200, &meta);
    let w1 = u32::from_le_bytes([d[4], d[5], d[6], d[7]]);
    assert_eq!((w1 >> 22) & 0x3, 0x3, "CCMP security type in word 1");
    // A plaintext frame leaves the field clear.
    let plain = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 0, sec_type: 0 };
    let dp = desc::frame(200, &plain);
    let w1p = u32::from_le_bytes([dp[4], dp[5], dp[6], dp[7]]);
    assert_eq!((w1p >> 22) & 0x3, 0, "no security type on a plaintext frame");
}

#[test]
fn ring_avail_reserves_one_descriptor() {
    let s = TxState::new(TX_DESC_COUNT);
    assert_eq!(s.avail(), TX_DESC_COUNT - 1, "an empty ring has len-1 free");
    let mut full = TxState::new(4);
    full.wp = 3;
    full.rp = 0; // wp just behind rp: one reserved, zero free
    assert_eq!(full.avail(), 0, "a full ring reports zero free");
}

#[test]
fn ring_advance_wraps_at_the_end() {
    let mut s = TxState::new(4);
    s.wp = 3;
    s.advance();
    assert_eq!(s.wp, 0, "the write index wraps to 0");
}

#[test]
fn hardware_read_index_is_taken_modulo_length() {
    let mut s = TxState::new(TX_DESC_COUNT);
    s.set_hw_rp(TX_DESC_COUNT + 5);
    assert_eq!(s.rp, 5, "the hardware index wraps into range");
}

#[test]
fn setup_programs_the_ring_address_and_length() {
    let card = Card::new(0);
    program(&card, 0x1_2345_6000);
    let w = card.writes.borrow();
    assert!(w.contains(&(REG_TXBD_DESA_BEQ, 0x2345_6000)), "low 32 bits of the ring address");
    assert!(w.contains(&(REG_TXBD_DESA_BEQ + 4, 0x1)), "high 32 bits of the ring address");
    assert!(w.contains(&(REG_TXBD_NUM_BEQ, TX_DESC_COUNT)), "ring length");
}

#[test]
fn enqueue_lays_the_frame_publishes_the_descriptor_and_kicks() {
    let card = Card::new(0);
    let ring = MockDma::new(TX_DESC_COUNT as usize * 16, 0x4000_0000);
    let buffers = MockDma::new(TX_DESC_COUNT as usize * TX_BUF_STRIDE, 0x5000_0000);
    let mut state = TxState::new(TX_DESC_COUNT);
    let frame = [0xC3u8; 100];
    let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 7, sec_type: 0 };

    assert!(enqueue(&card, &ring, &buffers, &mut state, &frame, &meta), "frame is queued");

    // The descriptor and frame land in slot 0's buffer.
    let buf = buffers.mem.borrow();
    let expect_desc = desc::frame(frame.len(), &meta);
    assert_eq!(&buf[..48], &expect_desc[..], "TX descriptor at slot start");
    assert_eq!(&buf[48..48 + frame.len()], &frame[..], "frame body after the descriptor");
    // The buffer descriptor for slot 0 points at the slot's buffer.
    let expect_bd = crate::ring::bufdesc::pair(
        buffers.device_addr(),
        TX_DESC_SIZE as usize,
        frame.len(),
        false,
    );
    assert_eq!(&ring.mem.borrow()[..16], &expect_bd[..], "buffer descriptor published");
    // The write index advanced and the queue was kicked with it.
    assert_eq!(state.wp, 1, "write index advanced");
    let w = card.writes.borrow();
    assert!(w.contains(&(REG_TXBD_IDX_BEQ, 1)), "queue kicked with the new write index");
}

#[test]
fn enqueue_refuses_a_full_ring() {
    // Hardware read index one behind the write index leaves no free descriptors.
    let card = Card::new(0);
    let ring = MockDma::new(TX_DESC_COUNT as usize * 16, 0x4000_0000);
    let buffers = MockDma::new(TX_DESC_COUNT as usize * TX_BUF_STRIDE, 0x5000_0000);
    let mut state = TxState::new(TX_DESC_COUNT);
    state.wp = TX_DESC_COUNT - 1; // hw_rp = 0 -> full
    let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 0, sec_type: 0 };
    assert!(!enqueue(&card, &ring, &buffers, &mut state, &[0u8; 64], &meta), "a full ring refuses");
}

#[test]
fn enqueue_refuses_an_oversized_frame() {
    let card = Card::new(0);
    let ring = MockDma::new(TX_DESC_COUNT as usize * 16, 0x4000_0000);
    let buffers = MockDma::new(TX_DESC_COUNT as usize * TX_BUF_STRIDE, 0x5000_0000);
    let mut state = TxState::new(TX_DESC_COUNT);
    let big = vec![0u8; TX_BUF_STRIDE]; // no room for the descriptor
    let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 0, sec_type: 0 };
    assert!(
        !enqueue(&card, &ring, &buffers, &mut state, &big, &meta),
        "an oversized frame refuses"
    );
}
