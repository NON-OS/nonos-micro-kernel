// NONOS Operating System (AGPL-3.0-or-later)
//! A deadline is reached exactly when the current time is at or past its end,
//! so a timeout fires neither early nor one tick late.

use crate::time::Deadline;

#[test]
fn a_deadline_is_past_only_at_or_after_its_end() {
    let d = Deadline::at(1000);
    assert!(!d.is_past(0));
    assert!(!d.is_past(999));
    assert!(d.is_past(1000)); // boundary: reached counts as past
    assert!(d.is_past(1001));
    assert!(d.is_past(u64::MAX));
}

#[test]
fn a_zero_deadline_is_immediately_past() {
    assert!(Deadline::at(0).is_past(0));
}
