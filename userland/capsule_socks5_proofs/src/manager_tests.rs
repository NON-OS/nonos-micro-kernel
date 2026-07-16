// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the tunneled-connection manager.

use crate::manager::{Manager, MAX_CONNS};

#[test]
fn open_assigns_nonzero_id_and_maps_back() {
    let mut m = Manager::new();
    let id = m.open(7).expect("first open must succeed");
    assert_ne!(id, 0, "id zero is reserved for no connection");
    assert_eq!(m.socket_of(id), Some(7));
    assert_eq!(m.count(), 1);
}

#[test]
fn ids_do_not_repeat_across_opens() {
    let mut m = Manager::new();
    let a = m.open(1).unwrap();
    let b = m.open(2).unwrap();
    let c = m.open(3).unwrap();
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
}

#[test]
fn close_by_id_frees_the_slot_and_returns_socket() {
    let mut m = Manager::new();
    let id = m.open(42).unwrap();
    assert_eq!(m.close(id), Some(42));
    assert_eq!(m.socket_of(id), None, "closed id no longer maps");
    assert_eq!(m.count(), 0);
    assert_eq!(m.close(id), None, "double close is a no-op");
}

#[test]
fn close_by_socket_returns_id_for_tunnel_teardown() {
    let mut m = Manager::new();
    let id = m.open(99).unwrap();
    assert_eq!(m.close_socket(99), Some(id));
    assert_eq!(m.socket_of(id), None);
    assert_eq!(m.close_socket(99), None, "unknown socket returns none");
}

#[test]
fn full_table_refuses_new_client() {
    let mut m = Manager::new();
    for s in 0..MAX_CONNS as u32 {
        assert!(m.open(s).is_some(), "slot {s} must fit");
    }
    assert_eq!(m.count(), MAX_CONNS);
    assert_eq!(m.open(1000), None, "over capacity is refused, not grown");
}

#[test]
fn freed_slot_is_reused() {
    let mut m = Manager::new();
    for s in 0..MAX_CONNS as u32 {
        m.open(s).unwrap();
    }
    let victim = m.open(1000);
    assert_eq!(victim, None);
    // Free one, then a new client fits again.
    assert!(m.close_socket(5).is_some());
    let reused = m.open(2000);
    assert!(reused.is_some(), "a freed slot admits a new client");
    assert_eq!(m.socket_of(reused.unwrap()), Some(2000));
}

#[test]
fn unknown_id_maps_to_nothing() {
    let m = Manager::new();
    assert_eq!(m.socket_of(1), None);
    assert_eq!(m.socket_of(0), None);
}

#[test]
fn send_sequence_starts_at_zero_and_advances_per_connection() {
    let mut m = Manager::new();
    let a = m.open(1).unwrap();
    let b = m.open(2).unwrap();
    assert_eq!(m.next_seq(a), Some(0), "first send is sequence zero");
    assert_eq!(m.next_seq(a), Some(1));
    assert_eq!(m.next_seq(a), Some(2));
    // Each connection keeps its own sequence.
    assert_eq!(m.next_seq(b), Some(0), "a second connection starts fresh");
    assert_eq!(m.next_seq(a), Some(3));
    // An unknown connection has no sequence.
    assert_eq!(m.next_seq(999), None);
}

#[test]
fn a_reopened_slot_resets_the_sequence() {
    let mut m = Manager::new();
    let a = m.open(1).unwrap();
    assert_eq!(m.next_seq(a), Some(0));
    assert_eq!(m.next_seq(a), Some(1));
    m.close(a);
    let b = m.open(2).unwrap();
    assert_eq!(m.next_seq(b), Some(0), "a fresh connection does not inherit a sequence");
}
