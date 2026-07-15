// NONOS Operating System (AGPL-3.0-or-later)
//! The picker must honour priority (handoff, then calibrated, then fresh) and
//! must evaluate the lower-priority sources lazily, so a fresh TSC calibration
//! or RTC read runs only when the earlier sources are absent.

use crate::resolve::pick_nonzero;
use core::cell::Cell;

#[test]
fn the_handoff_value_wins_and_no_fallback_runs() {
    let ran = Cell::new(false);
    let v = pick_nonzero(
        42,
        || {
            ran.set(true);
            7
        },
        || {
            ran.set(true);
            9
        },
    );
    assert_eq!(v, 42);
    assert!(!ran.get(), "lower-priority sources must not be evaluated");
}

#[test]
fn a_zero_handoff_falls_back_to_the_calibrated_value_without_running_fresh() {
    let fresh_ran = Cell::new(false);
    let v = pick_nonzero(0, || 7, || {
        fresh_ran.set(true);
        9
    });
    assert_eq!(v, 7);
    assert!(!fresh_ran.get(), "the fresh source must not run when calibrated is available");
}

#[test]
fn a_zero_handoff_and_calibrated_falls_back_to_fresh() {
    assert_eq!(pick_nonzero(0, || 0, || 9), 9);
}
