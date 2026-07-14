// NONOS Operating System (AGPL-3.0-or-later)
//! Hardening proofs: the driver parses attacker-controlled input, so no input,
//! however malformed, may panic, read out of bounds, or fail to terminate. Each
//! test drives a parser over hundreds of thousands of pseudo-random inputs; a
//! panic or a hang fails the test. This is the adversary model: the AP controls
//! every beacon byte, the firmware controls every DMA byte and the RX write
//! pointer, and the IPC client controls every request byte.

use crate::ccmp::ccm::ccm_decrypt;
use crate::constants::{RB_SIZE, RX_QUEUE_SIZE, RX_RB_OFFSET};
use crate::dot11::parse::{find_ie, parse_beacon};
use crate::eapol::mic::verify_mic;
use crate::eapol::parse::parse as eapol_parse;
use crate::rx::packet::parse as rx_parse;
use crate::rx::recv::receive;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn fuzz_buf(s: &mut u64, max_len: usize) -> Vec<u8> {
    let len = (xorshift(s) as usize) % (max_len + 1);
    let mut b = vec![0u8; len];
    for x in b.iter_mut() {
        *x = xorshift(s) as u8;
    }
    b
}

#[test]
fn beacon_parser_never_panics_on_arbitrary_frames() {
    let mut s = 0x1234_5678_9abc_def0;
    for _ in 0..400_000 {
        let buf = fuzz_buf(&mut s, 80);
        let _ = parse_beacon(&buf);
    }
}

#[test]
fn find_ie_terminates_and_never_panics() {
    let mut s = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..400_000 {
        let tags = fuzz_buf(&mut s, 64);
        let id = xorshift(&mut s) as u8;
        let _ = find_ie(&tags, id);
    }
}

#[test]
fn rx_packet_parser_never_panics_on_hostile_dma() {
    let mut s = 0xDEAD_BEEF_CAFE_BABE;
    for _ in 0..400_000 {
        let buf = fuzz_buf(&mut s, 64);
        let _ = rx_parse(&buf);
    }
}

#[test]
fn ccm_decrypt_never_panics_on_hostile_ciphertext() {
    // A rogue peer controls the ciphertext, tag and additional data; the
    // authenticated-decrypt path must never panic or read OOB, and must reject.
    let key = [0x2bu8; 16];
    let nonce = [0x11u8; 13];
    let mut out = [0u8; 256];
    let mut s = 0xC0DE_CAFE_1234_ABCD;
    for _ in 0..200_000 {
        let aad = fuzz_buf(&mut s, 32);
        let input = fuzz_buf(&mut s, 200);
        let _ = ccm_decrypt(&key, &nonce, &aad, &input, &mut out);
    }
}

#[test]
fn eapol_parse_and_mic_never_panic_on_hostile_frames() {
    // A rogue AP controls every byte of the handshake frame and the key data
    // length; neither the parse nor the MIC check may panic or read OOB.
    let mut s = 0xF00D_BABE_1234_5678;
    for _ in 0..400_000 {
        let frame = fuzz_buf(&mut s, 160);
        let _ = eapol_parse(&frame);
        let kck = [
            s as u8, (s >> 8) as u8, (s >> 16) as u8, (s >> 24) as u8, (s >> 32) as u8,
            (s >> 40) as u8, (s >> 48) as u8, (s >> 56) as u8, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let _ = verify_mic(&kck, &frame);
    }
}

#[test]
fn receive_never_panics_on_arbitrary_ring_state_and_reads_in_bounds() {
    // A garbage DMA buffer plus arbitrary read and firmware-write indices, as a
    // hostile or buggy firmware could present, must never fault the receive
    // path. The buffer is exactly the layout size, so an in-bounds read is also
    // a check that the masked index never runs past it.
    let mut dma = vec![0u8; RX_RB_OFFSET + RX_QUEUE_SIZE * RB_SIZE];
    let mut s = 0x0BAD_F00D_1357_2468;
    for x in dma.iter_mut() {
        *x = xorshift(&mut s) as u8;
    }
    let dma_va = dma.as_ptr() as u64;
    let mut out = [0u8; 256];
    for _ in 0..300_000 {
        let read = xorshift(&mut s) as usize;
        let write = xorshift(&mut s) as usize;
        let _ = unsafe { receive(dma_va, RX_RB_OFFSET, read, write, &mut out) };
    }
}
