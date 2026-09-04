// NONOS Operating System (AGPL-3.0-or-later)
use crate::watch::{Watch, SILENCE_MS};

// The exit watch decides when a silent exit is walked away from. The rule
// under proof: only an unproven, undirected exit that has been sent to and
// answered nothing for the whole budget rotates; delivery, configuration,
// or having sent nothing each veto it on their own.

#[test]
fn a_fresh_exit_never_rotates_before_anything_is_sent() {
    let w = Watch::new();
    assert!(!w.should_rotate(0));
    assert!(!w.should_rotate(i64::MAX));
}

#[test]
fn silence_past_the_budget_rotates_and_a_moment_less_does_not() {
    let mut w = Watch::new();
    w.on_send(1_000);
    assert!(!w.should_rotate(1_000 + SILENCE_MS - 1));
    assert!(w.should_rotate(1_000 + SILENCE_MS));
}

#[test]
fn the_budget_runs_from_the_first_send_not_the_newest() {
    let mut w = Watch::new();
    w.on_send(1_000);
    w.on_send(1_000 + SILENCE_MS - 1);
    assert!(w.should_rotate(1_000 + SILENCE_MS), "a later send must not reset the clock");
}

#[test]
fn one_delivery_proves_the_exit_for_good() {
    let mut w = Watch::new();
    w.on_send(1_000);
    w.on_delivered();
    assert!(!w.should_rotate(1_000 + SILENCE_MS * 100));
}

#[test]
fn a_configured_exit_is_never_rotated_away() {
    let mut w = Watch::new();
    w.configured = true;
    w.on_send(1_000);
    assert!(!w.should_rotate(1_000 + SILENCE_MS * 100));
}

#[test]
fn rotation_starts_the_next_exit_with_a_clean_record() {
    let mut w = Watch::new();
    w.on_send(1_000);
    assert!(w.should_rotate(1_000 + SILENCE_MS));
    w.on_rotate();
    assert!(!w.should_rotate(1_000 + SILENCE_MS * 2), "no send yet on the new exit");
    w.on_send(50_000);
    assert!(!w.should_rotate(50_000 + SILENCE_MS - 1));
    assert!(w.should_rotate(50_000 + SILENCE_MS));
}

#[test]
fn uptime_zero_edge_does_not_wedge_the_first_send() {
    // A send stamped at uptime 0 is indistinguishable from "never sent",
    // so the watch treats it as unset; the next send stamps for real. The
    // cost is one extra budget window on a machine that sends in the very
    // first millisecond of its life, never a wedge.
    let mut w = Watch::new();
    w.on_send(0);
    assert!(!w.should_rotate(SILENCE_MS * 2));
    w.on_send(5);
    assert!(w.should_rotate(5 + SILENCE_MS));
}
