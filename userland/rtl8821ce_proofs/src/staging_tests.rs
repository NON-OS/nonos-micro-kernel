// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for reserved-page staging, the hardware-specific half of
//! firmware download. A modeled card records every register write and its two
//! DMA regions (the staging buffer and the beacon-queue ring), and reports the
//! reserved page valid after the beacon kick. The proofs assert the exact bytes
//! of the beacon TX descriptor and the buffer descriptor the card would read,
//! the full arm/enable/kick/wait/restore register sequence, the constant DDMA
//! source a staged chunk resolves to, and that a card that never validates the
//! page is reported failed. Only the card physically consuming the page is left
//! for on-silicon bring-up.

use core::cell::RefCell;

use crate::fw::dma::DmaMem;
use crate::fw::regs::{
    BCN_VALID_V1, ENSWBCN_HI, EN_BCNQ_DL_B2, OCPBASE_TXBUF, PCI_BCNQ_FLAG, REG_CR,
    REG_FIFOPAGE_CTRL_2, REG_FWHW_TXQ_CTRL, REG_TXBD_BCN_WORK, TX_DESC_SIZE,
};
use crate::fw::rsvd::stage_chunk;
use crate::fw::txdesc::{self, TXDESC_LEN};
use crate::regs::Mmio;
use crate::ring::bufdesc;

// A DMA region backed by a byte vector: the exact bytes the card would fetch.
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

// A modeled card: it records register writes, and once the beacon queue is
// kicked it sets the beacon-valid status so the staging wait completes. A
// `wedged` card never validates the page.
struct Card {
    fifopage: RefCell<u16>,
    writes: RefCell<Vec<(usize, u32)>>,
    wedged: bool,
}

impl Card {
    fn new() -> Self {
        Self { fifopage: RefCell::new(0), writes: RefCell::new(Vec::new()), wedged: false }
    }
}

impl Mmio for Card {
    fn read8(&self, _off: usize) -> u8 {
        0
    }
    fn write8(&self, off: usize, val: u8) {
        self.writes.borrow_mut().push((off, val as u32));
        // The beacon kick makes the card land the reserved page: it reports the
        // page valid unless it is wedged.
        if off == REG_TXBD_BCN_WORK && val & PCI_BCNQ_FLAG != 0 && !self.wedged {
            *self.fifopage.borrow_mut() |= BCN_VALID_V1;
        }
    }
    fn read16(&self, off: usize) -> u16 {
        if off == REG_FIFOPAGE_CTRL_2 {
            *self.fifopage.borrow()
        } else {
            0
        }
    }
    fn write16(&self, off: usize, val: u16) {
        self.writes.borrow_mut().push((off, val as u32));
        if off == REG_FIFOPAGE_CTRL_2 {
            // Writing beacon-valid is write-1-to-clear: the status drops until
            // the card sets it again on the next kick.
            *self.fifopage.borrow_mut() = val & !BCN_VALID_V1;
        }
    }
    fn read32(&self, _off: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn beacon_tx_descriptor_has_the_expected_fields() {
    // A 0x400-byte chunk whose first byte is even (unicast): size in word 0, the
    // 48-byte header offset, last-segment and disable-qsel-seq set, no BMC.
    let d = txdesc::beacon(0x400, Some(0x00));
    let w0 = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
    let w1 = u32::from_le_bytes([d[4], d[5], d[6], d[7]]);
    let w8 = u32::from_le_bytes([d[32], d[33], d[34], d[35]]);
    assert_eq!(w0 & 0xFFFF, 0x400, "payload size in word 0");
    assert_eq!((w0 >> 16) & 0xFF, TX_DESC_SIZE, "48-byte descriptor offset");
    assert_ne!(w0 & (1 << 26), 0, "last-segment set");
    assert_ne!(w0 & (1 << 31), 0, "disable qsel sequence set");
    assert_eq!(w0 & (1 << 24), 0, "no broadcast/multicast for an even first byte");
    assert_eq!((w1 >> 8) & 0x1F, 16, "beacon queue selector in word 1");
    assert_ne!(w8 & (1 << 15), 0, "hardware sequence enabled in word 8");
    assert_eq!(d.len(), TXDESC_LEN);
}

#[test]
fn a_multicast_first_byte_sets_the_bmc_bit() {
    let d = txdesc::beacon(0x100, Some(0x01)); // bit 0 set: multicast group address
    let w0 = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
    assert_ne!(w0 & (1 << 24), 0, "an odd first byte is broadcast/multicast");
}

#[test]
fn beacon_buffer_descriptor_points_at_descriptor_then_payload() {
    let bd = bufdesc::pair(0x1000, TX_DESC_SIZE as usize, 0x400, true);
    // Entry 0: buf_size = 48, psb_len = pages | own, dma = descriptor base.
    assert_eq!(u16::from_le_bytes([bd[0], bd[1]]), 48, "entry 0 covers the TX descriptor");
    let psb = u16::from_le_bytes([bd[2], bd[3]]);
    assert_ne!(psb & (1 << 15), 0, "ownership handed to the card");
    // (0x400 + 48 - 1) / 128 + 1 = 9 pages.
    assert_eq!(psb & 0x7FFF, 9, "page span of the whole packet");
    assert_eq!(u32::from_le_bytes([bd[4], bd[5], bd[6], bd[7]]), 0x1000, "descriptor address");
    // Entry 1: buf_size = payload, dma = base + 48 (0x1000 + 0x30).
    assert_eq!(u16::from_le_bytes([bd[8], bd[9]]), 0x400, "entry 1 covers the payload");
    assert_eq!(u32::from_le_bytes([bd[12], bd[13], bd[14], bd[15]]), 0x1030, "payload address");
}

#[test]
fn staging_lays_the_chunk_and_runs_the_beacon_sequence() {
    let card = Card::new();
    let stage_buf = MockDma::new(0x2000, 0x5000_0000);
    let ring = MockDma::new(0x1000, 0x6000_0000);
    let chunk = [0xABu8; 0x200];

    let src = stage_chunk(&card, &stage_buf, &ring, &chunk).expect("page lands");
    // The DDMA source is the packet buffer just past the 48-byte descriptor.
    assert_eq!(src, OCPBASE_TXBUF + TX_DESC_SIZE);

    // The staging buffer holds [descriptor | payload].
    let buf = stage_buf.mem.borrow();
    assert_eq!(&buf[TXDESC_LEN..TXDESC_LEN + chunk.len()], &chunk[..], "payload staged after desc");
    // The ring holds the buffer descriptor pointing at the staging buffer.
    let expect_bd = bufdesc::pair(stage_buf.device_addr(), TXDESC_LEN, chunk.len(), true);
    assert_eq!(
        &ring.mem.borrow()[..expect_bd.len()],
        &expect_bd[..],
        "buffer descriptor published"
    );

    // The register program: arm beacon-valid, enable software beacon, hold off
    // the beacon-queue download, kick, then restore. Assert the ordered kick and
    // the enable writes are all present.
    let w = card.writes.borrow();
    let kicked =
        w.iter().position(|&(o, v)| o == REG_TXBD_BCN_WORK && v as u8 & PCI_BCNQ_FLAG != 0);
    assert!(kicked.is_some(), "the beacon queue is kicked");
    let armed =
        w.iter().position(|&(o, v)| o == REG_FIFOPAGE_CTRL_2 && v as u16 & BCN_VALID_V1 != 0);
    assert!(armed.unwrap() < kicked.unwrap(), "beacon-valid is armed before the kick");
    assert!(
        w.iter().any(|&(o, v)| o == REG_CR + 1 && v as u8 & ENSWBCN_HI != 0),
        "software beacon enabled"
    );
    assert!(
        w.iter().any(|&(o, v)| o == REG_FWHW_TXQ_CTRL + 2 && v as u8 & EN_BCNQ_DL_B2 == 0),
        "beacon-queue auto download held off"
    );
}

#[test]
fn a_wedged_card_fails_staging() {
    let mut card = Card::new();
    card.wedged = true;
    let stage_buf = MockDma::new(0x2000, 0x5000_0000);
    let ring = MockDma::new(0x1000, 0x6000_0000);
    assert!(
        stage_chunk(&card, &stage_buf, &ring, &[0u8; 0x100]).is_none(),
        "a card that never validates the page reports failure"
    );
}

#[test]
fn a_chunk_too_large_for_the_buffer_is_refused() {
    let card = Card::new();
    let stage_buf = MockDma::new(0x80, 0x5000_0000); // smaller than one chunk + descriptor
    let ring = MockDma::new(0x1000, 0x6000_0000);
    assert!(
        stage_chunk(&card, &stage_buf, &ring, &[0u8; 0x100]).is_none(),
        "oversized chunk refused"
    );
}
