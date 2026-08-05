// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that stream bytes coming back off the mixnet are put in order: a
//! chunk that arrives before the one in front of it is held rather than
//! passed on, chunks are handed over only once their gap is filled, two
//! connections do not see each other's bytes, a repeat of an already
//! delivered position is not delivered twice, the far end closing is reported
//! with the bytes that closed it, and forgetting a connection drops what was
//! still held for it.

use crate::inbox::Inbox;

#[test]
fn chunks_in_order_come_straight_back() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"hello ");
    inbox.accept(1, 1, false, b"world");
    let (bytes, closed) = inbox.drain(1);
    assert_eq!(bytes, b"hello world");
    assert!(!closed);
}

#[test]
fn a_chunk_that_arrives_early_waits_for_the_gap() {
    let mut inbox = Inbox::default();
    // The mixnet delays every packet separately, so the second half can and
    // does land first.
    inbox.accept(1, 1, false, b"world");
    let (bytes, _) = inbox.drain(1);
    assert!(bytes.is_empty(), "nothing is handed over across a gap");

    inbox.accept(1, 0, false, b"hello ");
    let (bytes, _) = inbox.drain(1);
    assert_eq!(bytes, b"hello world", "the gap filling releases both");
}

#[test]
fn connections_do_not_see_each_others_bytes() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"one");
    inbox.accept(2, 0, false, b"two");
    assert_eq!(inbox.drain(1).0, b"one");
    assert_eq!(inbox.drain(2).0, b"two");
}

#[test]
fn a_position_already_delivered_is_not_delivered_again() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"abc");
    assert_eq!(inbox.drain(1).0, b"abc");
    // A retransmit, or a duplicate the mixnet delivered twice.
    inbox.accept(1, 0, false, b"abc");
    assert!(inbox.drain(1).0.is_empty(), "the stream has moved past it");
    inbox.accept(1, 1, false, b"def");
    assert_eq!(inbox.drain(1).0, b"def");
}

#[test]
fn a_close_is_reported_with_the_bytes_that_carried_it() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"body");
    inbox.accept(1, 1, true, b" end");
    let (bytes, closed) = inbox.drain(1);
    assert_eq!(bytes, b"body end");
    assert!(closed, "the far end finished");
}

#[test]
fn a_close_behind_a_gap_is_not_reported_early() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 1, true, b" end");
    let (bytes, closed) = inbox.drain(1);
    assert!(bytes.is_empty());
    assert!(!closed, "a close cannot overtake the bytes in front of it");
}

#[test]
fn forgetting_a_connection_drops_what_was_held() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 1, false, b"world");
    inbox.forget(1);
    inbox.accept(1, 0, false, b"hello ");
    let (bytes, _) = inbox.drain(1);
    assert_eq!(bytes, b"hello ", "the held tail went with the connection");
}
