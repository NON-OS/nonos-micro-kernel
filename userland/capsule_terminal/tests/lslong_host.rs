// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
// Host-side harness for the `ls -l` columns: the mtime stamp, the byte total
// and the `.`/`..` rows. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/lslong_host.rs -o /tmp/lslong_host && /tmp/lslong_host
//
// The modules are pulled in by path so this cannot pass against a copy that
// has drifted from what the capsule runs.

extern crate alloc;

#[path = "../src/term/util/format_u64.rs"]
mod format_u64;

mod term {
    pub mod util {
        pub use crate::format_u64::format_u64;
    }
}

#[path = "../src/command/builtin/fs/ls_date.rs"]
mod ls_date;
#[path = "../src/command/builtin/fs/ls_num.rs"]
mod ls_num;
#[path = "../src/command/builtin/fs/ls_long.rs"]
mod ls_long;
#[path = "../src/command/builtin/fs/ls_dots.rs"]
mod ls_dots;
#[path = "../src/command/builtin/fs/ls_total.rs"]
mod ls_total;

use ls_date::stamp;
use ls_dots::{dot_names, dot_rows};
use ls_long::Row;
use ls_total::total_line;

fn at(ms: u64) -> String {
    String::from_utf8(stamp(ms)).expect("the stamp is ascii")
}

fn file(size: u64) -> Row {
    Row { name: b"f".to_vec(), size, is_dir: false, mtime: 0, writable: true }
}

fn total(sizes: &[u64], human: bool) -> String {
    let rows: Vec<Row> = sizes.iter().map(|&s| file(s)).collect();
    String::from_utf8(total_line(&rows, human)).expect("the total is ascii")
}

fn names(dots: &[Vec<u8>]) -> Vec<String> {
    dots.iter().map(|n| String::from_utf8(n.clone()).unwrap()).collect()
}

#[test]
fn the_epoch_itself_renders() {
    assert_eq!(at(1), "Jan  1 00:00");
}

/// The reason the day is space-padded rather than zero-padded: single digit
/// days must line up under the two digit ones without reading as octal.
#[test]
fn single_digit_days_are_space_padded() {
    assert_eq!(at(1_755_000_000_000), "Aug 12 12:00");
    assert_eq!(at(1_754_006_400_000), "Aug  1 00:00");
}

#[test]
fn known_epochs_render_their_civil_date() {
    for (ms, want) in [
        (1_755_594_600_000u64, "Aug 19 09:10"),
        (946_684_800_000, "Jan  1 00:00"),
        (1_000_000_000_000, "Sep  9 01:46"),
        (1_234_567_890_000, "Feb 13 23:31"),
        (2_147_483_647_000, "Jan 19 03:14"),
    ] {
        assert_eq!(at(ms), want, "{ms}");
    }
}

/// Leap days are where an era-based conversion goes wrong if the century rule
/// is missed: 2000 and 2024 are leap years, 2100 is not.
#[test]
fn leap_days_land_on_february_twentynine() {
    assert_eq!(at(951_782_400_000), "Feb 29 00:00");
    assert_eq!(at(1_709_164_800_000), "Feb 29 00:00");
    assert_eq!(at(1_709_251_200_000), "Mar  1 00:00");
    assert_eq!(at(4_107_456_000_000), "Feb 28 00:00");
    assert_eq!(at(4_107_542_400_000), "Mar  1 00:00");
}

#[test]
fn month_boundaries_do_not_slip_a_day() {
    for (ms, want) in [
        (1_735_689_599_000u64, "Dec 31 23:59"),
        (1_735_689_600_000, "Jan  1 00:00"),
        (1_738_367_999_000, "Jan 31 23:59"),
        (1_738_368_000_000, "Feb  1 00:00"),
    ] {
        assert_eq!(at(ms), want, "{ms}");
    }
}

/// Every stamp is the same width or the name column walks.
#[test]
fn every_stamp_is_twelve_bytes_including_the_unknown_one() {
    assert_eq!(stamp(0).len(), 12);
    assert_eq!(at(0), "           -");
    for ms in [1u64, 1_755_594_600_000, 4_107_542_400_000, 253_370_764_800_000] {
        assert_eq!(stamp(ms).len(), 12, "{ms}");
    }
}

#[test]
fn the_total_is_the_summed_byte_size() {
    assert_eq!(total(&[], false), "total 0");
    assert_eq!(total(&[1, 2, 3], false), "total 6");
    assert_eq!(total(&[4096, 900], false), "total 4996");
}

#[test]
fn the_total_follows_the_human_flag() {
    assert_eq!(total(&[1024, 1024], true), "total 2.0K");
    assert_eq!(total(&[1_048_576], true), "total 1.0M");
    assert_eq!(total(&[512], true), "total 512B");
}

/// A directory big enough to overflow the sum would otherwise wrap to a small
/// total, which reads as truth.
#[test]
fn the_total_saturates_instead_of_wrapping() {
    assert_eq!(total(&[u64::MAX, u64::MAX], false), total(&[u64::MAX], false));
}

#[test]
fn dots_appear_only_under_dash_a() {
    assert!(dot_names(b"/home/", false).is_empty());
    assert!(dot_names(b"/", false).is_empty());
    assert_eq!(names(&dot_names(b"/home/", true)), vec!["./", "../"]);
}

/// The root has no parent to name, so `..` there would point at itself.
#[test]
fn the_root_has_no_parent_row() {
    assert_eq!(names(&dot_names(b"/", true)), vec!["./"]);
}

#[test]
fn dot_rows_claim_nothing_the_vfs_did_not_report() {
    let rows = dot_rows(&dot_names(b"/home/", true));
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert!(row.is_dir);
        assert_eq!(row.size, 0);
        assert_eq!(row.mtime, 0);
        assert!(!row.writable);
    }
}

#[test]
fn a_dot_row_renders_as_a_directory_with_no_date() {
    let rows = dot_rows(&dot_names(b"/home/", true));
    let line = String::from_utf8(ls_long::long_row(&rows[0], false)).unwrap();
    assert_eq!(line, "dr--         0            - ./");
}
