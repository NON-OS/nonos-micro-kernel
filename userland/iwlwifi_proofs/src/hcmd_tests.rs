// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the host-command queue. The header/sequence encoding, the ring
//! advance and the TFD fill are pure, so they are checked exactly. `send`
//! composes a command into a modeled DMA buffer and rings a modeled doorbell,
//! so the bytes it lays down and the exact register write are validated with no
//! hardware.

use core::cell::RefCell;

use crate::constants::{
    CMD_AREA_OFFSET, CMD_QUEUE_ID, CMD_RING_OFFSET, CMD_SLOT_SIZE, HBUS_TARG_WRPTR, TFD_QUEUE_SIZE,
};
use crate::hcmd::header::{cmd_header, make_sequence, CMD_HEADER_LEN};
use crate::hcmd::ring::advance;
use crate::hcmd::send::{send_host_command, CmdQueue};
use crate::hcmd::tfd::{fill_single, TFD_SIZE};
use crate::regs::Mmio;

struct Rec {
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Mmio for Rec {
    fn read32(&self, _off: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn cmd_header_and_sequence_encode_correctly() {
    let seq = make_sequence(4, 7);
    assert_eq!(seq, (4u16 << 8) | 7, "queue in high byte, index in low byte");
    let h = cmd_header(0x9c, 0x00, seq);
    assert_eq!(h.len(), CMD_HEADER_LEN);
    assert_eq!(h[0], 0x9c);
    assert_eq!(h[1], 0x00);
    assert_eq!(u16::from_le_bytes([h[2], h[3]]), seq);
    // the queue field is masked to 5 bits
    assert_eq!(make_sequence(0xFF, 3), (0x1F << 8) | 3);
}

#[test]
fn ring_advance_wraps_at_queue_size() {
    assert_eq!(advance(0), 1);
    assert_eq!(advance(TFD_QUEUE_SIZE - 1), 0, "the last index wraps to zero");
    let mut p = 0usize;
    for _ in 0..TFD_QUEUE_SIZE {
        p = advance(p);
    }
    assert_eq!(p, 0, "a full lap returns to the start");
}

#[test]
fn tfd_fill_encodes_address_and_length() {
    let mut tfd = [0xEEu8; TFD_SIZE];
    let addr = 0x1_2345_6789u64;
    assert!(fill_single(&mut tfd, addr, 7));
    assert_eq!(tfd[3], 1, "one transfer buffer");
    let lo = u32::from_le_bytes([tfd[4], tfd[5], tfd[6], tfd[7]]);
    assert_eq!(lo, (addr & 0xFFFF_FFFF) as u32);
    let hi_n_len = u16::from_le_bytes([tfd[8], tfd[9]]);
    assert_eq!(hi_n_len & 0xF, ((addr >> 32) & 0xF) as u16, "high address bits");
    assert_eq!((hi_n_len >> 4) & 0x0FFF, 7, "the 12-bit length");
    // an over-long transfer is rejected
    assert!(!fill_single(&mut tfd, addr, 0x1000));
}

#[test]
fn send_host_command_lays_down_command_tfd_and_doorbell() {
    let mut dma = vec![0u8; CMD_AREA_OFFSET + TFD_QUEUE_SIZE * CMD_SLOT_SIZE];
    let dma_va = dma.as_mut_ptr() as u64;
    let dma_dev = 0x1_0000_0000u64;
    let rec = Rec { writes: RefCell::new(Vec::new()) };
    let payload = [0xAAu8, 0xBB, 0xCC];

    let q = CmdQueue {
        dma_user_va: dma_va,
        dma_device_addr: dma_dev,
        ring_off: CMD_RING_OFFSET,
        cmd_off: CMD_AREA_OFFSET,
        queue: CMD_QUEUE_ID,
    };
    let next = unsafe { send_host_command(&rec, &q, 0, 0x9c, 0x00, &payload) }.unwrap();
    assert_eq!(next, 1, "the write pointer advanced by one");

    let slot = CMD_AREA_OFFSET;
    assert_eq!(dma[slot], 0x9c, "command id in the slot");
    assert_eq!(dma[slot + 1], 0x00, "group");
    assert_eq!(
        u16::from_le_bytes([dma[slot + 2], dma[slot + 3]]),
        make_sequence(CMD_QUEUE_ID, 0),
        "sequence for queue and index 0"
    );
    assert_eq!(&dma[slot + 4..slot + 7], &payload, "payload after the header");

    let tfd = CMD_RING_OFFSET;
    assert_eq!(dma[tfd + 3], 1, "one transfer buffer in the TFD");
    let lo = u32::from_le_bytes([dma[tfd + 4], dma[tfd + 5], dma[tfd + 6], dma[tfd + 7]]);
    assert_eq!(lo, ((dma_dev + slot as u64) & 0xFFFF_FFFF) as u32, "TFD points at the command");
    let hi_n_len = u16::from_le_bytes([dma[tfd + 8], dma[tfd + 9]]);
    assert_eq!((hi_n_len >> 4) & 0x0FFF, (CMD_HEADER_LEN + payload.len()) as u16, "TFD length");

    let w = rec.writes.borrow();
    assert_eq!(w.len(), 1, "exactly one doorbell write");
    assert_eq!(w[0].0, HBUS_TARG_WRPTR);
    assert_eq!(w[0].1, ((CMD_QUEUE_ID as u32) << 8) | 1, "queue and advanced index");
}
