// NONOS Operating System (AGPL-3.0-or-later)
//! The gate must skip the validity window while the clock still reads uptime
//! (so runtime capsules are not falsely rejected as NotYetValid) and enforce it
//! once the clock is a plausible wall clock.

use crate::validity_clock::validity_now_ms;

#[test]
fn uptime_before_the_clock_is_set_skips_the_window() {
    assert_eq!(validity_now_ms(0), None);
    assert_eq!(validity_now_ms(5_000), None); // ~5 s of uptime
    assert_eq!(validity_now_ms(60_000_000), None); // ~16 h of uptime
}

#[test]
fn a_plausible_wall_clock_enforces_the_window() {
    const FLOOR: u64 = 1_577_836_800_000; // 2020-01-01
    assert_eq!(validity_now_ms(FLOOR - 1), None);
    assert_eq!(validity_now_ms(FLOOR), Some(FLOOR));
    let jul_2026 = 1_752_000_000_000u64;
    assert_eq!(validity_now_ms(jul_2026), Some(jul_2026));
}
