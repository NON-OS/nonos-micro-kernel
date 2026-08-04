//! What one packet carries, in the order a recipient reads it.

use crate::ack::PADDED_ADDRESS_BYTES;
use crate::payload::{build_payload, packet_shared_key, PACKET_KEY_BYTES};

const RECIPIENT_KEY: [u8; 32] = [7u8; 32];

/// The order is the contract. A recipient cannot be told where one part ends
/// and the next begins, so it reads by offset: the ack, the public half of
/// this packet's key agreement, then the sealed fragment.
#[test]
fn a_payload_is_laid_out_in_reading_order() {
    let ack = vec![0x11u8; PADDED_ADDRESS_BYTES + 64];
    let fragment = vec![0x22u8; 100];
    let out = build_payload(&ack, &RECIPIENT_KEY, &fragment).expect("a payload must build");

    assert_eq!(&out[..ack.len()], &ack[..], "the ack leads");
    assert_eq!(out.len(), ack.len() + 32 + fragment.len(), "one key, then the fragment");
}

/// The fragment must not travel in the clear. The last hop peels its own
/// layer off the packet, and only the recipient is meant to read what is
/// inside.
#[test]
fn the_fragment_is_sealed() {
    let ack = vec![0u8; PADDED_ADDRESS_BYTES];
    let fragment = vec![0x22u8; 64];
    let out = build_payload(&ack, &RECIPIENT_KEY, &fragment).expect("a payload must build");
    let sealed = &out[ack.len() + 32..];
    assert_ne!(sealed, &fragment[..], "the fragment went out in the clear");
}

/// A key agreed per packet is what stops two packets to the same recipient
/// from sharing one, which would link them.
#[test]
fn every_packet_agrees_its_own_key() {
    let (first_public, first_key) = packet_shared_key(&RECIPIENT_KEY).expect("a key must agree");
    let (second_public, second_key) = packet_shared_key(&RECIPIENT_KEY).expect("a key must agree");

    assert_ne!(first_public, second_public, "the public half repeated");
    assert_ne!(first_key, second_key, "two packets shared a key");
    assert_eq!(first_key.len(), PACKET_KEY_BYTES);
}

/// Sealing the same fragment twice must not produce the same bytes, or the
/// ciphertext itself would say the two packets carry the same thing.
#[test]
fn the_same_fragment_seals_differently_each_time() {
    let ack = vec![0u8; PADDED_ADDRESS_BYTES];
    let fragment = vec![0x22u8; 64];
    let first = build_payload(&ack, &RECIPIENT_KEY, &fragment).expect("a payload must build");
    let second = build_payload(&ack, &RECIPIENT_KEY, &fragment).expect("a payload must build");
    assert_ne!(first, second);
}
