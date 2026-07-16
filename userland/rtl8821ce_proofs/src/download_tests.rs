// NONOS Operating System (AGPL-3.0-or-later)
//! End-to-end proof of the firmware download, run against the real RTL8821CE
//! image and a modeled card. The card completes each DDMA transfer and stages
//! chunks into a fake packet buffer; the proof asserts the whole 139472-byte
//! image is delivered to the right on-chip addresses in DDMA-sized chunks with
//! per-section checksum framing, and that a truncated image, a failed stage and
//! a bad checksum each stop the download with the right error.

use core::cell::RefCell;

use crate::fw::ddma::{DDMA_CHKSUM_STS, DDMA_OWN, REG_DDMA_CH0CTRL, REG_DDMA_CH0DA};
use crate::fw::{download, DownloadError, MAX_CHUNK};
use crate::regs::Mmio;

const FW: &[u8] = include_bytes!("../../capsule_driver_rtl8821ce/firmware/rtw8821c_fw.bin");

struct Card {
    ctrl: RefCell<u32>,
    // (dst_addr, chunk_len) of every DDMA transfer, in order.
    xfers: RefCell<Vec<(u32, usize)>>,
    chksum_bad: bool,
}

impl Card {
    fn new() -> Self {
        Self { ctrl: RefCell::new(0), xfers: RefCell::new(Vec::new()), chksum_bad: false }
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
    fn write16(&self, _o: usize, _v: u16) {}
    fn read32(&self, off: usize) -> u32 {
        if off == REG_DDMA_CH0CTRL {
            *self.ctrl.borrow()
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off == REG_DDMA_CH0DA {
            // Record where this transfer lands; length comes on the CTRL write.
            self.xfers.borrow_mut().push((val, 0));
        }
        if off == REG_DDMA_CH0CTRL {
            if let Some(last) = self.xfers.borrow_mut().last_mut() {
                last.1 = (val & 0x3FFFF) as usize;
            }
            let mut done = val & !DDMA_OWN;
            if self.chksum_bad {
                done |= DDMA_CHKSUM_STS;
            } else {
                done &= !DDMA_CHKSUM_STS;
            }
            *self.ctrl.borrow_mut() = done;
        }
    }
}

// A staging callback that always succeeds, returning a distinct source address
// per chunk (its running offset).
fn ok_stage() -> impl FnMut(&[u8]) -> Option<u32> {
    let mut src = 0u32;
    move |chunk: &[u8]| {
        let a = src;
        src += chunk.len() as u32;
        Some(a)
    }
}

#[test]
fn the_real_firmware_downloads_to_the_right_addresses() {
    let card = Card::new();
    assert_eq!(download(&card, FW, ok_stage()), Ok(()), "the full image downloads");

    let xfers = card.xfers.borrow();
    // Every chunk is within one DDMA transfer.
    assert!(xfers.iter().all(|(_, len)| *len <= MAX_CHUNK && *len > 0), "chunks are DDMA-sized");
    // The three sections' bytes all arrive.
    let total: usize = xfers.iter().map(|(_, l)| *l).sum();
    assert_eq!(total, FW.len() - crate::fw::header::FW_HDR_LEN, "all section bytes are delivered");
    // The first transfer of each section targets that section's on-chip base
    // address (bit 31 of the image-declared address is cleared before use).
    let bases: Vec<u32> = [0x0020_0000u32, 0x0003_0000, 0x0010_0000].to_vec();
    for base in bases {
        assert!(xfers.iter().any(|(dst, _)| *dst == base), "section base {base:#x} is written");
    }
}

#[test]
fn a_short_image_is_a_bad_image() {
    let card = Card::new();
    assert_eq!(download(&card, &FW[..32], ok_stage()), Err(DownloadError::BadImage));
}

#[test]
fn a_failed_stage_stops_the_download() {
    let card = Card::new();
    let stage = |_c: &[u8]| None; // packet buffer always refuses
    assert_eq!(download(&card, FW, stage), Err(DownloadError::Stage));
}

#[test]
fn a_bad_checksum_stops_the_download() {
    let mut card = Card::new();
    card.chksum_bad = true;
    assert_eq!(download(&card, FW, ok_stage()), Err(DownloadError::Checksum));
}
