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

extern crate alloc;

use alloc::{string::String, vec::Vec};

// Pure millisecond arithmetic against a fixed day, not a calendar: the journal
// clock and `mk_time_millis` are the same wall clock, so an age in days is all
// the bucketing needs and no timezone has to be invented.
pub const DAY_MS: u64 = 86_400_000;
const MIN_MS: u64 = 60_000;
const HOUR_MS: u64 = 3_600_000;

/// Age buckets, coarsest last. `Older` keeps anything past a fortnight rather
/// than dropping it, so the surface never silently loses a journal entry.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Bucket {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    Older,
}

/// Every bucket with its section label, in drawn order.
pub const BUCKETS: [(Bucket, &str); 5] = [
    (Bucket::Today, "TODAY"),
    (Bucket::Yesterday, "YESTERDAY"),
    (Bucket::ThisWeek, "EARLIER THIS WEEK"),
    (Bucket::LastWeek, "LAST WEEK"),
    (Bucket::Older, "OLDER"),
];

/// A timestamp ahead of `now` (clock skew across a persisted journal) saturates
/// to a zero age rather than underflowing into `Older`.
pub fn bucket_of(now_ms: u64, then_ms: u64) -> Bucket {
    match now_ms.saturating_sub(then_ms) / DAY_MS {
        0 => Bucket::Today,
        1 => Bucket::Yesterday,
        2..=6 => Bucket::ThisWeek,
        7..=13 => Bucket::LastWeek,
        _ => Bucket::Older,
    }
}

/// The heading used when there is no wall clock to bucket against. Every age
/// would saturate to zero and read as TODAY, which would be a lie, so the whole
/// journal goes under one section that says the clock is missing instead.
pub const NO_CLOCK: &str = "RECENTLY OPENED (CLOCK UNAVAILABLE)";

/// The one grouping pass the Recents surface draws from, over rows the surface
/// has already filtered. Only non-empty buckets come back, so the painter never
/// has to decide whether a section exists.
pub fn group<'a>(now_ms: u64, rows: &[(u64, &'a str)]) -> Vec<(&'static str, Vec<(u64, &'a str)>)> {
    if now_ms == 0 {
        return match rows.is_empty() {
            true => Vec::new(),
            false => alloc::vec![(NO_CLOCK, rows.to_vec())],
        };
    }
    let mut out = Vec::new();
    for (bucket, label) in BUCKETS {
        let got: Vec<(u64, &str)> =
            rows.iter().copied().filter(|(ms, _)| bucket_of(now_ms, *ms) == bucket).collect();
        if !got.is_empty() {
            out.push((label, got));
        }
    }
    out
}

/// A short "how long ago" label on the same saturating age.
pub fn rel_time(now_ms: u64, then_ms: u64) -> String {
    let age = now_ms.saturating_sub(then_ms);
    if age < MIN_MS {
        return String::from("just now");
    }
    if age < HOUR_MS {
        return alloc::format!("{}m ago", age / MIN_MS);
    }
    if age < DAY_MS {
        return alloc::format!("{}h ago", age / HOUR_MS);
    }
    if age < DAY_MS * 2 {
        return String::from("yesterday");
    }
    alloc::format!("{}d ago", age / DAY_MS)
}

/// The directory a path sits in, with its trailing separator; a top-level path
/// reports the root.
pub fn parent_of(path: &str) -> &str {
    match path.trim_end_matches('/').rfind('/') {
        Some(i) => &path[..=i],
        None => "/",
    }
}
