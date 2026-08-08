//! What comes back, and the ways it must not be read.

use crate::message::{pad_to_packets, Fragment};
use crate::reply::{collect, reply_body, TAG_REPLY_DATA, TYPE_REPLY};

/// Build the fragments a reply of `body` would arrive as.
fn fragments_of(body: &[u8], set_id: i32, width: usize) -> Vec<Vec<u8>> {
    let mut message = vec![TYPE_REPLY, TAG_REPLY_DATA];
    message.extend_from_slice(body);
    let padded = pad_to_packets(message, width).expect("a real width must pad");
    let total = (padded.len() / width) as u8;
    padded
        .chunks(width)
        .enumerate()
        .map(|(i, chunk)| {
            Fragment { set_id, total, current: (i + 1) as u8 }.into_bytes(chunk)
        })
        .collect()
}

#[test]
fn a_single_fragment_reply_completes() {
    let mut slot = None;
    let pieces = fragments_of(b"HTTP/1.1 200 OK", 5, 256);
    assert_eq!(pieces.len(), 1);
    let done = collect(&mut slot, &pieces[0]).expect("one fragment completes one message");
    assert_eq!(reply_body(&done).expect("the body must strip"), b"HTTP/1.1 200 OK");
}

/// The mixnet does not preserve order, so a fragment is placed by the
/// position in its own header rather than by when it turned up.
#[test]
fn fragments_reassemble_out_of_order() {
    let body: Vec<u8> = (0..900u32).map(|i| (i % 251) as u8).collect();
    let pieces = fragments_of(&body, 9, 128);
    assert!(pieces.len() > 2, "this reply has to span packets");

    let mut slot = None;
    let mut done = None;
    // Deliver back to front, which is a real arrival order for a mixnet.
    for piece in pieces.iter().rev() {
        if let Some(message) = collect(&mut slot, piece) {
            done = Some(message);
        }
    }
    let message = done.expect("every fragment arrived, so the message must complete");
    assert_eq!(reply_body(&message).expect("the body must strip"), &body[..]);
}

/// A repeated fragment must not count twice, or a replay could complete a
/// message that is still missing a piece.
#[test]
fn a_repeated_fragment_does_not_complete_a_message() {
    let body: Vec<u8> = vec![3u8; 600];
    let pieces = fragments_of(&body, 11, 128);
    assert!(pieces.len() > 2);

    let mut slot = None;
    for _ in 0..pieces.len() * 2 {
        assert!(collect(&mut slot, &pieces[0]).is_none(), "one fragment is not a message");
    }
}

/// A message claiming to be something else has either been tampered with or
/// belongs to a protocol this does not speak.
#[test]
fn a_message_of_the_wrong_type_is_refused() {
    let mut slot = None;
    let mut pieces = fragments_of(b"body", 13, 256);
    // Flip the type byte, which sits first in the message.
    let header = crate::message::UNLINKED_HEADER_LEN;
    pieces[0][header] = TYPE_REPLY + 1;
    let done = collect(&mut slot, &pieces[0]).expect("it still reassembles");
    assert!(reply_body(&done).is_none(), "a foreign type must not be read past");
}

/// A new message replaces one that can never complete, rather than being
/// dropped behind it.
#[test]
fn a_new_message_displaces_an_unfinished_one() {
    let mut slot = None;
    let stalled = fragments_of(&vec![1u8; 600], 21, 128);
    assert!(collect(&mut slot, &stalled[0]).is_none());

    let fresh = fragments_of(b"second", 22, 256);
    let done = collect(&mut slot, &fresh[0]).expect("the newer message must complete");
    assert_eq!(reply_body(&done).expect("the body must strip"), b"second");
}
