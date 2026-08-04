//! What a read too small for its frame leaves behind.

use crate::server::handlers::mixnet_frame::MAX_BODY;
use crate::server::handlers::mixnet_residual::{release, store, take};
use crate::sockets::SocketKey;

fn key(handle: u32) -> SocketKey {
    SocketKey { pid: 42, handle }
}

/// The regression. A frame is off the mixnet queue once read, so a caller
/// with a smaller buffer used to lose the difference for good.
#[test]
fn a_short_read_keeps_the_rest_of_its_frame() {
    let k = key(1);
    let body: Vec<u8> = (0..MAX_BODY).map(|i| (i % 251) as u8).collect();

    let mut first = [0u8; 256];
    let taken = body.len().min(first.len());
    first.copy_from_slice(&body[..taken]);
    assert!(store(k, &body[taken..]));

    let mut seen = first.to_vec();
    loop {
        let mut next = [0u8; 256];
        let n = take(k, &mut next);
        if n == 0 {
            break;
        }
        seen.extend_from_slice(&next[..n]);
    }
    assert_eq!(seen, body, "the reader must see the whole frame across reads");
    release(k);
}

/// Order matters as much as completeness: a stream read out of sequence is
/// corruption, not a short read.
#[test]
fn the_rest_comes_back_in_order() {
    let k = key(2);
    assert!(store(k, b"cdef"));
    let mut out = [0u8; 2];
    assert_eq!(take(k, &mut out), 2);
    assert_eq!(&out, b"cd");
    assert_eq!(take(k, &mut out), 2);
    assert_eq!(&out, b"ef");
    assert_eq!(take(k, &mut out), 0);
    release(k);
}

/// One socket's leftovers must never be served to another.
#[test]
fn a_socket_only_reads_its_own_leftovers() {
    let a = key(3);
    let b = key(4);
    assert!(store(a, b"aaaa"));
    let mut out = [0u8; 4];
    assert_eq!(take(b, &mut out), 0, "another socket must see nothing");
    assert_eq!(take(a, &mut out), 4);
    release(a);
    release(b);
}

/// A closed socket must not leave bytes for whatever is handed its number
/// next.
#[test]
fn closing_drops_what_was_held() {
    let k = key(5);
    assert!(store(k, b"stale"));
    release(k);
    let mut out = [0u8; 8];
    assert_eq!(take(k, &mut out), 0);
}

/// A reader that keeps coming up short must reuse its own slot rather than
/// eat its way through the table, which is the sequence a receive runs: drain
/// what is held, pull the next frame, keep what did not fit.
#[test]
fn a_repeat_short_reader_holds_one_slot() {
    let k = key(6);
    let mut out = [0u8; 3];
    for _ in 0..64 {
        assert!(store(k, b"xyz"), "a socket must always fit its own remainder");
        assert_eq!(take(k, &mut out), 3);
    }
    release(k);
}

/// Bytes already waiting come earlier in the stream, so a store that would
/// bury them has to be refused rather than reorder the stream.
#[test]
fn unread_bytes_are_not_overwritten() {
    let k = key(8);
    assert!(store(k, b"first"));
    assert!(!store(k, b"second"), "a held remainder must not be buried");
    let mut out = [0u8; 5];
    assert_eq!(take(k, &mut out), 5);
    assert_eq!(&out, b"first");
    release(k);
}

/// Nothing left over is not a failure to store.
#[test]
fn an_exact_read_leaves_nothing() {
    let k = key(7);
    assert!(store(k, &[]));
    let mut out = [0u8; 4];
    assert_eq!(take(k, &mut out), 0);
}
