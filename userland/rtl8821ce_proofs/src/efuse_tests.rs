// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the efuse read: the physical dump drives the efuse control register
//! exactly (address in bits 8..18, poll the flag, take the low byte) and returns
//! every byte; the de-shuffle unpacks both the one-byte and two-byte block header
//! formats, honours the word-enable skips, and stops at an erased header; and the
//! whole read extracts the RF front-end option, package bit and cut version the
//! PHY bring-up needs. Driven against a modeled efuse, so the exact register
//! protocol and the packed-format decode are checked without a card.

use std::cell::Cell;

use crate::efuse::{deshuffle, dump_physical, read};
use crate::regs::Mmio;

const REG_EFUSE_CTRL: usize = 0x0030;
const REG_CHIP_VER: usize = 0x00F0;
const EF_FLAG: u32 = 1 << 31;

// A modeled efuse: a backing physical map addressed through the control register,
// plus the cut version reported by the config register.
struct MockEfuse {
    phys: [u8; 512],
    cut: u8,
    addr: Cell<usize>,
}

impl MockEfuse {
    fn new(phys: [u8; 512], cut: u8) -> Self {
        Self { phys, cut, addr: Cell::new(0) }
    }
}

impl Mmio for MockEfuse {
    fn read8(&self, off: usize) -> u8 {
        // The 2.5V LDO enable at REG_LDO_EFUSE_CTRL + 3 reads set, so the read
        // path exercises clearing it.
        if off == 0x0037 {
            0x80
        } else {
            0
        }
    }
    fn write8(&self, _o: usize, _v: u8) {}
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, _o: usize, _v: u16) {}
    fn read32(&self, off: usize) -> u32 {
        if off == REG_EFUSE_CTRL {
            // The flag reads set immediately with the addressed byte in the low
            // bits, as a completed read would.
            EF_FLAG | self.phys[self.addr.get()] as u32
        } else if off == REG_CHIP_VER {
            (self.cut as u32) << 12
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off == REG_EFUSE_CTRL {
            self.addr.set(((val >> 8) & 0x3ff) as usize);
        }
    }
}

#[test]
fn the_physical_dump_reads_every_byte_through_the_control_register() {
    let mut phys = [0u8; 512];
    phys[0] = 0xA0;
    phys[5] = 0x5A;
    phys[300] = 0xC3;
    phys[511] = 0xFE;
    let m = MockEfuse::new(phys, 0);
    let got = dump_physical(&m).expect("the modeled efuse completes every read");
    assert_eq!(got[0], 0xA0);
    assert_eq!(got[5], 0x5A);
    assert_eq!(got[300], 0xC3, "the address latched into bits 8..18 selects the byte");
    assert_eq!(got[511], 0xFE);
}

#[test]
fn deshuffle_one_byte_header_places_present_words_and_skips_absent() {
    // Block 3, word-enable 0b1010: words 0 and 2 present, 1 and 3 absent. Logical
    // base 3 << 3 = 24; word 0 -> 24, word 2 -> 28.
    let mut phys = [0xffu8; 512];
    phys[0] = 0x3A; // (3 << 4) | 0x0A
    phys[1] = 0x11;
    phys[2] = 0x22;
    phys[3] = 0x33;
    phys[4] = 0x44;
    phys[5] = 0xFF; // erased header ends the map
    let mut log = [0xffu8; 512];
    deshuffle(&phys, &mut log);
    assert_eq!(&log[24..30], &[0x11, 0x22, 0xff, 0xff, 0x33, 0x44], "word 1 stays erased");
}

#[test]
fn deshuffle_two_byte_header_addresses_high_blocks() {
    // Block 25 needs the two-byte header. hdr1 0x2F, hdr2 0x3D: block index 25,
    // word-enable 0b1101 (only word 1 present). Logical base 200; word 1 -> 202.
    let mut phys = [0xffu8; 512];
    phys[0] = 0x2F;
    phys[1] = 0x3D;
    phys[2] = 0xAB;
    phys[3] = 0xCD;
    phys[4] = 0xFF;
    let mut log = [0xffu8; 512];
    deshuffle(&phys, &mut log);
    assert_eq!(log[202], 0xAB, "word 1 of block 25 lands at logical 0xCA");
    assert_eq!(log[203], 0xCD);
    assert_eq!(log[200], 0xff, "word 0 was absent");
}

#[test]
fn read_extracts_rfe_pkg_and_cut() {
    // Place the RF front-end option byte 0x22 at logical 0xCA via block 25 word 1:
    // rfe = 0x22 & 0x1f = 2, package = bit 5 = 1. Cut version 0xB.
    let mut phys = [0xffu8; 512];
    phys[0] = 0x2F;
    phys[1] = 0x3D;
    phys[2] = 0x22;
    phys[3] = 0x00;
    phys[4] = 0xFF;
    let m = MockEfuse::new(phys, 0x0B);
    let info = read(&m).expect("the efuse reads");
    assert_eq!(info.rfe, 2, "RF front-end option is the low five bits");
    assert_eq!(info.pkg, 1, "package is bit five of the option byte");
    assert_eq!(info.cut, 0x0B, "cut version comes from the config register");
}
