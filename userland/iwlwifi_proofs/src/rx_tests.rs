// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the receive path. The packet framing and the ring math are pure,
//! so they are checked exactly. `receive` reads from a modeled DMA ring and is
//! checked against the packet a firmware would have posted, with no hardware.

use crate::constants::{RB_SIZE, RX_QUEUE_SIZE, RX_RB_OFFSET};
use crate::rx::packet::{parse, RX_HEADER_LEN};
use crate::rx::recv::receive;
use crate::rx::ring::{advance, has_packet};

/// Lay down an iwl_rx_packet in `buf`: len_n_flags counting the command header
/// plus the data, then cmd/group/sequence, then the data.
fn put_packet(buf: &mut [u8], cmd: u8, group: u8, seq: u16, data: &[u8]) {
    let after = 4 + data.len();
    buf[0..4].copy_from_slice(&(after as u32).to_le_bytes());
    buf[4] = cmd;
    buf[5] = group;
    buf[6..8].copy_from_slice(&seq.to_le_bytes());
    buf[8..8 + data.len()].copy_from_slice(data);
}

#[test]
fn parse_reads_command_group_sequence_and_payload() {
    let mut buf = [0u8; 64];
    put_packet(&mut buf, 0x9c, 0x01, 0x0407, &[0xDE, 0xAD, 0xBE, 0xEF]);
    let pkt = parse(&buf).unwrap();
    assert_eq!(pkt.cmd, 0x9c);
    assert_eq!(pkt.group, 0x01);
    assert_eq!(pkt.sequence, 0x0407);
    assert_eq!(pkt.payload, &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn parse_rejects_short_or_overrunning_packets() {
    assert!(parse(&[0u8; RX_HEADER_LEN - 1]).is_none(), "shorter than the header");
    let mut buf = [0u8; 16];
    // claim a length that runs past the buffer
    buf[0..4].copy_from_slice(&0x3FFFu32.to_le_bytes());
    assert!(parse(&buf).is_none(), "declared length overruns");
}

#[test]
fn ring_advance_and_has_packet() {
    assert_eq!(advance(RX_QUEUE_SIZE - 1), 0, "the read index wraps");
    assert!(!has_packet(3, 3), "caught up means empty");
    assert!(has_packet(3, 5), "behind the write index means a packet is waiting");
}

#[test]
fn receive_returns_the_posted_packet_and_advances() {
    let mut dma = vec![0u8; RX_RB_OFFSET + RX_QUEUE_SIZE * RB_SIZE];
    // firmware posted one packet into receive buffer 0.
    let rb0 = RX_RB_OFFSET;
    put_packet(&mut dma[rb0..], 0x88, 0x00, 0x1234, &[1, 2, 3]);
    let dma_va = dma.as_ptr() as u64;

    let mut out = [0u8; 64];
    let (cmd, group, seq, n, next) =
        unsafe { receive(dma_va, RX_RB_OFFSET, 0, 1, &mut out) }.unwrap();
    assert_eq!(cmd, 0x88);
    assert_eq!(group, 0x00);
    assert_eq!(seq, 0x1234);
    assert_eq!(n, 3);
    assert_eq!(&out[..3], &[1, 2, 3]);
    assert_eq!(next, 1, "the read index advanced");

    // an empty ring (read == write) yields nothing.
    assert!(unsafe { receive(dma_va, RX_RB_OFFSET, 1, 1, &mut out) }.is_none());
}
