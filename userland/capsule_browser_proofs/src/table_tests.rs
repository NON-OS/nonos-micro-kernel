// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for automatic table column widths (CSS 2.1 17.5.2). The
//! three regimes (room to spare, shrink between the totals, overflow below the
//! minima) plus the invariant that the widths fill the row exactly.

use crate::browser::layout::boxmodel::table_columns::column_widths;

#[test]
fn columns_take_their_max_and_share_the_surplus_by_weight() {
    // Exactly enough room: each column its max.
    assert_eq!(column_widths(&[100, 200], &[0, 0], 300), vec![100, 200]);
    // Surplus shared in proportion to the maxima (300 extra split 1:2).
    assert_eq!(column_widths(&[100, 200], &[0, 0], 600), vec![200, 400]);
    // Equal maxima split the surplus equally.
    assert_eq!(column_widths(&[100, 100], &[0, 0], 300), vec![150, 150]);
}

#[test]
fn columns_shrink_from_max_toward_min_across_their_range() {
    // total_max 400, total_min 150; take 100 of the 250 shrinkable span (40%).
    assert_eq!(column_widths(&[100, 300], &[50, 100], 250), vec![70, 180]);
    assert_eq!(column_widths(&[200, 200], &[50, 50], 300), vec![150, 150]);
}

#[test]
fn columns_fall_back_to_min_and_overflow_when_too_narrow() {
    // Not even room for the minima: each takes its min, the table overflows.
    assert_eq!(column_widths(&[100, 200], &[80, 120], 100), vec![80, 120]);
    // Exactly the minima.
    assert_eq!(column_widths(&[100, 200], &[80, 120], 200), vec![80, 120]);
}

#[test]
fn the_widths_fill_the_available_width_exactly() {
    for &avail in &[300, 450, 600, 1000] {
        let w = column_widths(&[100, 200, 50], &[10, 20, 5], avail);
        assert_eq!(w.iter().sum::<i32>(), avail, "columns tile the row at avail={avail}");
    }
    // Empty columns split the width; no column left at zero when there is room.
    assert_eq!(column_widths(&[0, 0], &[0, 0], 100), vec![50, 50]);
    assert!(column_widths(&[], &[], 100).is_empty());
}
