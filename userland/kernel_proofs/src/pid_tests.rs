// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the PID selection arithmetic that backs `allocate_tid`. The real
//! `choose_pid` is included and exercised directly; these assert the invariants
//! the SMP allocation race used to violate (a duplicate or zero PID).
use crate::pid_alloc::choose_pid;
use std::collections::BTreeSet;

#[test]
fn chosen_pid_and_counter_are_never_zero() {
    for current in [0u32, 1, 2, 100, u32::MAX - 2, u32::MAX - 1, u32::MAX] {
        let (pid, next) = choose_pid(current, |_| false).unwrap();
        assert_ne!(pid, 0, "a PID of 0 is never handed out");
        assert_ne!(next, 0, "the counter never wraps to 0");
    }
}

#[test]
fn counter_wraps_to_one_at_the_top() {
    let (_pid, next) = choose_pid(u32::MAX - 1, |_| false).unwrap();
    assert_eq!(next, 1, "the counter wraps to 1, skipping 0");
    let (_pid, next) = choose_pid(u32::MAX, |_| false).unwrap();
    assert_eq!(next, 1);
}

#[test]
fn active_pids_are_skipped() {
    let active: BTreeSet<u32> = [5, 6, 7, 8].into_iter().collect();
    let (pid, _next) = choose_pid(5, |p| active.contains(&p)).unwrap();
    assert!(!active.contains(&pid), "an active PID is never handed out");
    assert_eq!(pid, 9, "allocation skips the active run to the next free PID");
}

#[test]
fn repeated_allocation_yields_unique_nonzero_pids() {
    // The invariant the race broke: two callers must never get the same PID.
    // Model the serialized allocator handing out N ids, marking each active.
    let mut active: BTreeSet<u32> = BTreeSet::new();
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    let mut current = 1u32;
    for _ in 0..10_000 {
        let (pid, next) = choose_pid(current, |p| active.contains(&p)).expect("space available");
        assert_ne!(pid, 0);
        assert!(seen.insert(pid), "every allocated PID is unique");
        active.insert(pid);
        current = next;
    }
}

#[test]
fn a_fully_active_space_reports_exhaustion() {
    // If every probed candidate is active, allocation gives up rather than
    // looping forever or returning a live PID.
    assert!(choose_pid(1, |_| true).is_none(), "exhaustion returns None");
}
