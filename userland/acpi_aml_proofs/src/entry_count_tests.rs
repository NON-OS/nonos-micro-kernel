// NONOS Operating System (AGPL-3.0-or-later)
//! The SDT entry-count helper must never underflow: a table declaring a length
//! smaller than its header yields zero entries, not a near-2^64 count that would
//! drive an out-of-bounds read loop over firmware memory.

use crate::sdt_entry_count::{sdt_entry_count, MAX_TABLE_BYTES};

#[test]
fn a_length_below_the_header_yields_zero_not_an_underflow() {
    assert_eq!(sdt_entry_count(0, 36, 8), 0);
    assert_eq!(sdt_entry_count(35, 36, 8), 0);
    assert_eq!(sdt_entry_count(36, 36, 8), 0); // exactly the header, no entries
}

#[test]
fn entries_are_counted_by_size() {
    assert_eq!(sdt_entry_count(36 + 8 * 5, 36, 8), 5); // XSDT: five 8-byte pointers
    assert_eq!(sdt_entry_count(36 + 4 * 7, 36, 4), 7); // RSDT: seven 4-byte pointers
    assert_eq!(sdt_entry_count(36 + 8 * 3 + 3, 36, 8), 3); // trailing partial entry dropped
}

#[test]
fn a_zero_entry_size_is_safe() {
    assert_eq!(sdt_entry_count(1000, 36, 0), 0);
}

#[test]
fn the_table_cap_is_a_megabyte() {
    assert_eq!(MAX_TABLE_BYTES, 1 << 20);
}
