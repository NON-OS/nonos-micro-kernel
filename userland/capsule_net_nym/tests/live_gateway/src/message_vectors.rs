//! The message layer, against the shapes nym's own serializer produces.

use crate::message::{
    pad_to_packets, parse, repliable_data, unpad, Fragment, SENDER_TAG_SIZE, TAG_DATA,
    TYPE_REPLIABLE, UNLINKED_HEADER_LEN,
};

/// nym writes a set id with the top bit set, then the counts, then a zero
/// saying the set is not linked to another.
#[test]
fn a_fragment_header_matches_the_reference_layout() {
    let bytes = Fragment { set_id: 0x0102_0304, total: 3, current: 2 }.into_bytes(b"body");
    assert_eq!(&bytes[..UNLINKED_HEADER_LEN], &[0x81, 0x02, 0x03, 0x04, 3, 2, 0]);
    assert_eq!(&bytes[UNLINKED_HEADER_LEN..], b"body");
}

#[test]
fn a_fragment_header_round_trips() {
    let bytes = Fragment { set_id: 7, total: 1, current: 1 }.into_bytes(b"payload");
    let (header, payload) = parse(&bytes).expect("our own header must parse");
    assert_eq!(header.set_id, 7);
    assert_eq!(header.total, 1);
    assert_eq!(header.current, 1);
    assert_eq!(payload, b"payload");
}

/// Without the marker bit these are not fragment bytes, and reading them as
/// though they were would hand the caller a set id that was never sent.
#[test]
fn a_header_without_the_marker_is_refused() {
    let mut bytes = Fragment { set_id: 7, total: 1, current: 1 }.into_bytes(b"x");
    bytes[0] &= 0x7f;
    assert!(parse(&bytes).is_none());
}

/// Linked sets carry four more bytes this does not produce, so they are
/// refused rather than read as though the field were absent.
#[test]
fn a_linked_header_is_refused() {
    let mut bytes = Fragment { set_id: 7, total: 1, current: 1 }.into_bytes(b"x");
    bytes[6] = 1;
    assert!(parse(&bytes).is_none());
}

#[test]
fn a_fragment_out_of_range_is_refused() {
    let bytes = Fragment { set_id: 1, total: 2, current: 3 }.into_bytes(b"x");
    assert!(parse(&bytes).is_none());
}

/// The exact byte order nym reads a repliable data message in.
#[test]
fn a_repliable_message_matches_the_reference_layout() {
    let tag = [0xabu8; SENDER_TAG_SIZE];
    let surb = vec![0x11u8; 40];
    let out = repliable_data(&tag, core::slice::from_ref(&surb), b"GET /");

    assert_eq!(out[0], TYPE_REPLIABLE);
    assert_eq!(&out[1..1 + SENDER_TAG_SIZE], &tag);
    assert_eq!(out[1 + SENDER_TAG_SIZE], TAG_DATA);
    let count = &out[2 + SENDER_TAG_SIZE..6 + SENDER_TAG_SIZE];
    assert_eq!(count, &1u32.to_be_bytes(), "surb count is big endian");
    assert_eq!(&out[6 + SENDER_TAG_SIZE..46 + SENDER_TAG_SIZE], &surb[..]);
    assert_eq!(&out[46 + SENDER_TAG_SIZE..], b"GET /");
}

/// A request with no surbs attached is still well formed on the wire, and is
/// exactly the case an exit cannot answer.
#[test]
fn a_message_with_no_surbs_still_states_the_count() {
    let out = repliable_data(&[0u8; SENDER_TAG_SIZE], &[], b"hi");
    let count = &out[2 + SENDER_TAG_SIZE..6 + SENDER_TAG_SIZE];
    assert_eq!(count, &0u32.to_be_bytes());
}

/// Padding fills whole packets so every one on the wire is the same width.
#[test]
fn padding_fills_whole_packets() {
    let padded = pad_to_packets(vec![7u8; 10], 32).expect("a real packet size must pad");
    assert_eq!(padded.len(), 32);
    assert_eq!(padded[10], 1, "the marker says where the message stopped");
    assert!(padded[11..].iter().all(|&b| b == 0));

    let two = pad_to_packets(vec![7u8; 40], 32).expect("a real packet size must pad");
    assert_eq!(two.len(), 64);
}

/// A message that already lands on the boundary still gets a marker, or the
/// far end cannot tell the message from its padding.
#[test]
fn a_message_on_the_boundary_still_gets_a_marker() {
    let padded = pad_to_packets(vec![7u8; 32], 32).expect("a real packet size must pad");
    assert_eq!(padded.len(), 64);
    assert_eq!(padded[32], 1);
}

#[test]
fn padding_round_trips() {
    let message = vec![9u8; 100];
    let padded = pad_to_packets(message.clone(), 64).expect("a real packet size must pad");
    assert_eq!(unpad(&padded).expect("our own padding must strip"), &message[..]);
}

/// Bytes after the marker have to be padding. Anything else means this is not
/// the message that was sent.
#[test]
fn padding_with_a_dirty_tail_is_refused() {
    let mut padded = pad_to_packets(vec![9u8; 10], 32).expect("a real packet size must pad");
    let last = padded.len() - 1;
    padded[last] = 2;
    assert!(unpad(&padded).is_none());
}

/// The version a hop reads to know how the payload keys were built.
///
/// nym writes it as a big endian u16 behind a leading zero, so a version put
/// in the first byte names no version at all. Ours has to be the explicit
/// payload keys one, because that is how our keys are built.
#[test]
fn the_packet_version_is_one_a_hop_knows() {
    use crate::sphinx_root::sphinx::constants::PACKET_VERSION;

    assert_eq!(PACKET_VERSION[0], 0, "the leading byte is not part of the number");
    let value = u16::from_be_bytes([PACKET_VERSION[1], PACKET_VERSION[2]]);
    assert_eq!(value, 258, "explicit payload keys over standard X25519");
}

/// A packet has to leave room for a message after its overheads, and the
/// sizes it is derived from are not obviously large enough to see by eye.
#[test]
fn a_packet_has_room_for_a_message() {
    use crate::message::{FRAGMENT_PER_PACKET, PLAINTEXT_PER_PACKET};
    use crate::message::UNLINKED_HEADER_LEN;

    assert!(PLAINTEXT_PER_PACKET > 0);
    assert_eq!(FRAGMENT_PER_PACKET - PLAINTEXT_PER_PACKET, UNLINKED_HEADER_LEN);
    assert!(PLAINTEXT_PER_PACKET > 1024, "a packet should carry a useful request");
}

/// A short request is one packet, and its fragment says so.
#[test]
fn a_short_request_is_one_fragment() {
    use crate::message::{parse, prepare, SENDER_TAG_SIZE};

    let prepared = prepare(&[1u8; SENDER_TAG_SIZE], &[], b"GET / HTTP/1.1", 42)
        .expect("a short request must prepare");
    assert_eq!(prepared.fragments.len(), 1);

    let (header, _) = parse(&prepared.fragments[0]).expect("our own fragment must parse");
    assert_eq!(header.set_id, 42);
    assert_eq!(header.total, 1);
    assert_eq!(header.current, 1, "positions count from one");
}

/// Reply blocks are bulky, so a request carrying them spans packets. Every
/// piece has to be numbered in order or the far end cannot rebuild it.
#[test]
fn a_request_with_reply_blocks_spans_packets_in_order() {
    use crate::message::{parse, prepare, PLAINTEXT_PER_PACKET, SENDER_TAG_SIZE};

    let surbs: Vec<Vec<u8>> = (0..4).map(|_| vec![0xabu8; 900]).collect();
    let prepared = prepare(&[2u8; SENDER_TAG_SIZE], &surbs, b"GET /", 7)
        .expect("a request with blocks must prepare");
    assert!(prepared.fragments.len() > 1, "blocks this size do not fit one packet");

    for (index, fragment) in prepared.fragments.iter().enumerate() {
        let (header, payload) = parse(fragment).expect("every fragment must parse");
        assert_eq!(header.set_id, 7);
        assert_eq!(header.total as usize, prepared.fragments.len());
        assert_eq!(header.current as usize, index + 1);
        assert_eq!(payload.len(), PLAINTEXT_PER_PACKET, "every packet is the same width");
    }
}
