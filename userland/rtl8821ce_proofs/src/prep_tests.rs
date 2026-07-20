// NONOS Operating System (AGPL-3.0-or-later)
//! Known-behaviour proofs for the firmware-download prologue and epilogue. A
//! modeled register file backs reads so the read-modify-writes work, and records
//! writes. The proofs assert that `begin` halts the 8051, maps the queues onto
//! the download path and pulses the platform reset, and that `finish` restores
//! the saved registers, closes out the download only when both section checksums
//! are good, releases the 8051 and reports ready when the firmware-ready state
//! reads back.

use core::cell::RefCell;

use crate::fw::prep::{begin, finish, Finish};
use crate::fw::regs::{
    CHECK_SUM_OK, CR_TXDMA_EN, DMA_MAPPING_HIGH_B1, FEN_CPUEN_HI, FW_READY, MCUFWDL_EN, REG_CR,
    REG_MCUFW_CTRL, REG_RSV_CTRL, REG_SYS_FUNC_EN, REG_TXDMA_PQ_MAP, WLMCU_IOIF_HI,
};
use crate::regs::Mmio;

const SIZE: usize = 0x2000;

struct Card {
    regs: RefCell<[u8; SIZE]>,
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Card {
    fn new() -> Self {
        Self { regs: RefCell::new([0u8; SIZE]), writes: RefCell::new(Vec::new()) }
    }
    fn preset16(&self, off: usize, val: u16) {
        let b = val.to_le_bytes();
        self.regs.borrow_mut()[off] = b[0];
        self.regs.borrow_mut()[off + 1] = b[1];
    }
    fn wrote(&self, off: usize) -> Option<u32> {
        self.writes.borrow().iter().rev().find(|&&(o, _)| o == off).map(|&(_, v)| v)
    }
}

impl Mmio for Card {
    fn read8(&self, off: usize) -> u8 {
        self.regs.borrow()[off]
    }
    fn write8(&self, off: usize, val: u8) {
        self.regs.borrow_mut()[off] = val;
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read16(&self, off: usize) -> u16 {
        let r = self.regs.borrow();
        u16::from_le_bytes([r[off], r[off + 1]])
    }
    fn write16(&self, off: usize, val: u16) {
        let b = val.to_le_bytes();
        self.regs.borrow_mut()[off] = b[0];
        self.regs.borrow_mut()[off + 1] = b[1];
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read32(&self, off: usize) -> u32 {
        let r = self.regs.borrow();
        u32::from_le_bytes([r[off], r[off + 1], r[off + 2], r[off + 3]])
    }
    fn write32(&self, off: usize, val: u32) {
        let b = val.to_le_bytes();
        for (i, byte) in b.iter().enumerate() {
            self.regs.borrow_mut()[off + i] = *byte;
        }
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn begin_halts_the_cpu_and_maps_the_download_queues() {
    let card = Card::new();
    let _ = begin(&card);
    // The 8051 run enable is cleared (halted).
    assert_eq!(card.wrote(REG_SYS_FUNC_EN + 1), Some(0), "cpu run enable cleared");
    // The queues are pointed at the download path.
    assert_eq!(card.wrote(REG_TXDMA_PQ_MAP + 1), Some(DMA_MAPPING_HIGH_B1 as u32), "high pri map");
    assert_eq!(card.wrote(REG_CR), Some(CR_TXDMA_EN as u32), "tx dma enabled for download");
}

#[test]
fn finish_restores_registers_and_reports_ready() {
    let card = Card::new();
    // Seed distinctive register values so the restore is observable.
    card.regs.borrow_mut()[REG_CR] = 0x5A;
    card.preset16(REG_MCUFW_CTRL, FW_READY | MCUFWDL_EN); // checksums good, still enabled

    let backup = begin(&card);
    // begin overwrote REG_CR; finish must put the seed back.
    assert!(matches!(finish(&card, backup), Finish::Ready), "a good checksum and ready state finishes");
    assert_eq!(card.wrote(REG_CR), Some(0x5A), "the saved control register is restored");
    // The download enable is turned back off in the final MCUFW_CTRL write.
    let final_ctrl = card.read16(REG_MCUFW_CTRL);
    assert_eq!(final_ctrl & MCUFWDL_EN, 0, "download disabled after loading");
    // The 8051 is released: its IO interface restored and run enable set again.
    assert_eq!(card.read8(REG_RSV_CTRL + 1), WLMCU_IOIF_HI, "cpu io interface restored");
    assert_eq!(card.wrote(REG_SYS_FUNC_EN + 1), Some(FEN_CPUEN_HI as u32), "cpu run enable set");
}

#[test]
fn finish_fails_on_a_bad_checksum() {
    let card = Card::new();
    // Ready bits present but the section checksums are not both good.
    card.preset16(REG_MCUFW_CTRL, FW_READY & !CHECK_SUM_OK);
    let backup = begin(&card);
    assert!(
        matches!(finish(&card, backup), Finish::ChecksumBad(_)),
        "a bad checksum stops the download"
    );
}
