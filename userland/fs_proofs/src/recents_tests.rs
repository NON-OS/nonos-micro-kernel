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

//! The Recents grouping is the one part of that surface with no pixels in it,
//! so it is provable on the host. These pin the bucket boundaries, the clock
//! skew guard, and the reserved-namespace filter the UI depends on.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::fm_logic::recents_group::{bucket_of, group, parent_of, rel_time, Bucket, DAY_MS};
use crate::fm_logic::recents_tally::shown;

const NOW: u64 = 1_000 * DAY_MS;

fn entry(ms: u64, path: &str) -> (u64, String) {
    (ms, path.to_string())
}

fn labels(out: &[(&'static str, Vec<(u64, &str)>)]) -> Vec<&'static str> {
    out.iter().map(|(label, _)| *label).collect()
}

#[test]
fn bucket_boundaries_land_on_whole_elapsed_days() {
    assert!(bucket_of(NOW, NOW) == Bucket::Today);
    assert!(bucket_of(NOW, NOW - (DAY_MS - 1)) == Bucket::Today);
    assert!(bucket_of(NOW, NOW - DAY_MS) == Bucket::Yesterday);
    assert!(bucket_of(NOW, NOW - (2 * DAY_MS - 1)) == Bucket::Yesterday);
    assert!(bucket_of(NOW, NOW - 2 * DAY_MS) == Bucket::ThisWeek);
    assert!(bucket_of(NOW, NOW - 6 * DAY_MS) == Bucket::ThisWeek);
    assert!(bucket_of(NOW, NOW - 7 * DAY_MS) == Bucket::LastWeek);
    assert!(bucket_of(NOW, NOW - 13 * DAY_MS) == Bucket::LastWeek);
    assert!(bucket_of(NOW, NOW - 14 * DAY_MS) == Bucket::Older);
}

#[test]
fn a_future_timestamp_saturates_to_today_instead_of_wrapping_to_older() {
    assert!(bucket_of(NOW, NOW + 5 * DAY_MS) == Bucket::Today);
    assert!(bucket_of(0, u64::MAX) == Bucket::Today);
    assert_eq!(rel_time(0, u64::MAX), "just now");
}

#[test]
fn the_journal_view_drops_the_reserved_sidecar_namespace() {
    let entries = [
        entry(NOW, "/.files/tags"),
        entry(NOW, "/.files/favorites"),
        entry(NOW, "/.files/prefs"),
        entry(NOW, "/docs/report.txt"),
    ];
    let out = group(NOW, &shown(&entries, None));
    let rows: Vec<&str> = out.iter().flat_map(|(_, r)| r.iter().map(|(_, p)| *p)).collect();
    assert_eq!(rows, ["/docs/report.txt"]);
}

#[test]
fn a_reserved_only_journal_produces_no_sections_at_all() {
    let entries = [entry(NOW, "/.files/tags")];
    assert!(group(NOW, &shown(&entries, None)).is_empty());
}

#[test]
fn only_non_empty_buckets_come_back_and_they_keep_the_declared_order() {
    let entries = [
        entry(NOW - 10 * DAY_MS, "/old.txt"),
        entry(NOW, "/new.txt"),
        entry(NOW - 3 * DAY_MS, "/mid.txt"),
    ];
    assert_eq!(labels(&group(NOW, &shown(&entries, None))), ["TODAY", "EARLIER THIS WEEK", "LAST WEEK"]);
}

#[test]
fn every_entry_lands_in_exactly_one_bucket() {
    let entries: Vec<(u64, String)> =
        (0..30).map(|d| entry(NOW - d * DAY_MS, "/f.txt")).collect();
    let total: usize = group(NOW, &shown(&entries, None)).iter().map(|(_, r)| r.len()).sum();
    assert_eq!(total, entries.len());
}

#[test]
fn rel_time_crosses_its_units_without_a_gap() {
    assert_eq!(rel_time(NOW, NOW), "just now");
    assert_eq!(rel_time(NOW, NOW - 59_999), "just now");
    assert_eq!(rel_time(NOW, NOW - 60_000), "1m ago");
    assert_eq!(rel_time(NOW, NOW - 3_599_999), "59m ago");
    assert_eq!(rel_time(NOW, NOW - 3_600_000), "1h ago");
    assert_eq!(rel_time(NOW, NOW - (DAY_MS - 1)), "23h ago");
    assert_eq!(rel_time(NOW, NOW - DAY_MS), "yesterday");
    assert_eq!(rel_time(NOW, NOW - (2 * DAY_MS - 1)), "yesterday");
    assert_eq!(rel_time(NOW, NOW - 2 * DAY_MS), "2d ago");
}

#[test]
fn parent_of_reports_a_directory_with_its_separator() {
    assert_eq!(parent_of("/docs/report.txt"), "/docs/");
    assert_eq!(parent_of("/docs/sub/"), "/docs/");
    assert_eq!(parent_of("/top.txt"), "/");
    assert_eq!(parent_of("/"), "/");
    assert_eq!(parent_of("bare"), "/");
    assert_eq!(parent_of(""), "/");
}

#[test]
fn parent_of_never_cuts_a_multibyte_path_mid_character() {
    for path in ["/dösc/räpport.txt", "/日本/語.txt", "/emoji/🗂️/f"] {
        let parent = parent_of(path);
        assert!(path.starts_with(parent) || parent == "/");
    }
}
