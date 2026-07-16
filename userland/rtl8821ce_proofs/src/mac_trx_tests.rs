// NONOS Operating System (AGPL-3.0-or-later)
//! Proof for the TRX-config bring-up (`mac::init_trx_cfg`): the step that enables
//! the MAC's transmit and receive engines. A modeled device records every write
//! and, like the real card, clears the link-list-table auto-init bit once it is
//! triggered and reports its H2C queue empty. The proofs assert the exact rtw88
//! 8821c register program: the priority-to-queue map, the REG_CR engine enable
//! bracketing it, the per-queue page counts and reserved-page boundary, the
//! receive FIFO boundary, the LLT trigger, and the H2C queue addresses, and that
//! a card whose LLT never finishes or whose H2C ring is not free fails.

use core::cell::RefCell;

use crate::mac::{init_trx_cfg, reset_trx_dma};
use crate::regs::Mmio;

const REG_CR: usize = 0x0100;
const REG_TXDMA_PQ_MAP: usize = 0x010C;
const REG_RXFF_BNDY: usize = 0x011C;
const REG_FIFOPAGE_CTRL_2: usize = 0x0204;
const REG_AUTO_LLT_V1: usize = 0x0208;
const REG_RQPN_CTRL_2: usize = 0x022C;
const REG_FIFOPAGE_INFO_1: usize = 0x0230;
const REG_FIFOPAGE_INFO_5: usize = 0x0240;
const REG_H2C_HEAD: usize = 0x0244;
const REG_H2C_TAIL: usize = 0x0248;
const REG_BCNQ_BDNY_V1: usize = 0x0424;
const REG_H2CQ_CSR: usize = 0x1330;

struct Card {
    // Last value written to each byte-offset, for read-modify-write reads.
    store: RefCell<[u32; 0x2000]>,
    // Ordered log of (offset, value) writes, for order-sensitive assertions.
    log: RefCell<Vec<(usize, u32)>>,
    // A card whose auto-init LLT never clears.
    llt_stuck: bool,
    // A card whose H2C ring reports as not fully free.
    h2c_busy: bool,
}

impl Card {
    fn new() -> Self {
        Self {
            store: RefCell::new([0u32; 0x2000]),
            log: RefCell::new(Vec::new()),
            llt_stuck: false,
            h2c_busy: false,
        }
    }
    fn put(&self, off: usize, val: u32) {
        if off < 0x2000 {
            self.store.borrow_mut()[off] = val;
        }
        self.log.borrow_mut().push((off, val));
    }
    // The last value written to `off`.
    fn last(&self, off: usize) -> Option<u32> {
        self.log.borrow().iter().rev().find(|(o, _)| *o == off).map(|(_, v)| *v)
    }
    // Every value written to `off`, in order.
    fn seq(&self, off: usize) -> Vec<u32> {
        self.log.borrow().iter().filter(|(o, _)| *o == off).map(|(_, v)| *v).collect()
    }
}

impl Mmio for Card {
    fn read8(&self, off: usize) -> u8 {
        if off == REG_AUTO_LLT_V1 && !self.llt_stuck {
            // The hardware clears the auto-init bit when it finishes.
            return (self.store.borrow()[off] as u8) & 0xFE;
        }
        self.store.borrow().get(off).copied().unwrap_or(0) as u8
    }
    fn write8(&self, off: usize, val: u8) {
        self.put(off, val as u32);
    }
    fn read16(&self, off: usize) -> u16 {
        self.store.borrow().get(off).copied().unwrap_or(0) as u16
    }
    fn write16(&self, off: usize, val: u16) {
        self.put(off, val as u32);
    }
    fn read32(&self, off: usize) -> u32 {
        // The H2C write/read pointers read back equal, so the ring is fully free,
        // unless this card models a busy ring (write pointer advanced).
        if off == 0x10D4 && self.h2c_busy {
            return 0x10;
        }
        self.store.borrow().get(off).copied().unwrap_or(0)
    }
    fn write32(&self, off: usize, val: u32) {
        self.put(off, val as u32);
    }
}

#[test]
fn enables_the_trx_engines_with_the_exact_program() {
    let card = Card::new();
    assert!(init_trx_cfg(&card), "a healthy card comes up");

    // The priority-to-queue map for the 8821c PCIe row.
    assert_eq!(card.last(REG_TXDMA_PQ_MAP), Some(0xC5A0), "txdma priority-queue map");

    // REG_CR is cleared, then written with all eight engine-enable bits, in that
    // order, bracketing the queue map. The receive bits are what let it hear.
    let cr = card.seq(REG_CR);
    assert_eq!(cr, vec![0x00, 0xFF], "REG_CR cleared then MAC_TRX_ENABLE");

    // The 11ac beacon-queue-full marker.
    assert_eq!(card.last(REG_H2CQ_CSR), Some(0x8000_0000), "H2CQ full marker");

    // The per-queue page counts and the public-queue remainder.
    assert_eq!(card.last(REG_FIFOPAGE_INFO_1), Some(16), "hq pages");
    assert_eq!(card.last(REG_FIFOPAGE_INFO_5), Some(397), "pubq pages");

    // The reserved-page boundary (460) lands in the boundary registers.
    assert_eq!(card.last(REG_FIFOPAGE_CTRL_2), Some(460), "reserved boundary");
    assert_eq!(card.last(REG_BCNQ_BDNY_V1), Some(460), "beacon queue boundary");

    // The load-rqpn latch is set.
    assert_eq!(card.last(REG_RQPN_CTRL_2), Some(1 << 31), "load rqpn latch");

    // The receive FIFO boundary leaves room for the C2H buffer.
    assert_eq!(card.last(REG_RXFF_BNDY), Some(0x3EFF), "rx fifo boundary");

    // The H2C queue is pointed at its reserved pages.
    assert_eq!(card.last(REG_H2C_HEAD), Some(0xFA00), "h2c head page address");
    assert_eq!(card.last(REG_H2C_TAIL), Some(0xFE00), "h2c tail = head + size");
}

#[test]
fn reset_trx_dma_clears_pointers_and_resets_the_dma_interface() {
    let card = Card::new();
    // Seed the control register with reserved high bits so the reset is proven to
    // preserve them rather than overwrite the register.
    card.store.borrow_mut()[0x0300] = 0x0000_0004;
    reset_trx_dma(&card);
    // Ring read/write pointers cleared.
    assert_eq!(card.last(0x039C), Some(0xFFFF_FFFF), "ring pointers cleared");
    // H2C host and hardware indices cleared (bits 16 and 8).
    assert_eq!(card.last(0x1330).unwrap() & ((1 << 16) | (1 << 8)), (1 << 16) | (1 << 8));
    // DMA interface reset (bit 20) and receive tag enabled (bit 15), reserved bit
    // preserved.
    let ctrl = card.last(0x0300).unwrap();
    assert_eq!(ctrl & (1 << 20), 1 << 20, "trx dma interface reset");
    assert_eq!(ctrl & (1 << 15), 1 << 15, "receive tag enabled");
    assert_eq!(ctrl & 0x0000_0004, 0x0000_0004, "reserved bits preserved");
}

#[test]
fn a_stuck_link_list_table_fails_bring_up() {
    let card = Card { llt_stuck: true, ..Card::new() };
    assert!(!init_trx_cfg(&card), "an LLT that never finishes fails");
}

#[test]
fn an_unfree_h2c_ring_fails_bring_up() {
    let card = Card { h2c_busy: true, ..Card::new() };
    assert!(!init_trx_cfg(&card), "an H2C ring that is not fully free fails");
}
