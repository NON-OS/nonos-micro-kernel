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

use crate::browser::css::computed::{GridTrack, MAX_GRID_COLS};

use super::matching_paren::matching_paren;
use super::one_track::one_track;

// grid-template-columns track list: lengths, percentages, Nfr, auto,
// minmax(a, b) and repeat(n, track). Tracks past the cap drop; a list with
// no parseable track is rejected.
pub(super) fn parse_grid_tracks(value: &str, em: u32) -> Option<([GridTrack; MAX_GRID_COLS], u8)> {
    let mut cols = [GridTrack::Fr(1); MAX_GRID_COLS];
    let mut n = 0usize;
    let mut rest = value.trim();
    while !rest.is_empty() && n < MAX_GRID_COLS {
        rest = rest.trim_start();
        let low = rest.to_ascii_lowercase();
        if low.starts_with("repeat(") {
            let after = &rest[7..];
            let close = matching_paren(after);
            if let Some(body) = after.get(..close) {
                if let Some(comma) = body.find(',') {
                    if let Some(track) = one_track(&body[comma + 1..], em) {
                        let count =
                            body[..comma].trim().parse::<usize>().unwrap_or(0).min(MAX_GRID_COLS);
                        for _ in 0..count {
                            if n >= MAX_GRID_COLS {
                                break;
                            }
                            cols[n] = track;
                            n += 1;
                        }
                    }
                }
            }
            rest = after.get(close + 1..).unwrap_or("");
            continue;
        }
        // minmax carries a comma and spaces, so it spans to its close paren.
        let end = if low.starts_with("minmax(") {
            (7 + matching_paren(&rest[7..]) + 1).min(rest.len())
        } else {
            rest.find(char::is_whitespace).unwrap_or(rest.len())
        };
        if let Some(track) = one_track(&rest[..end], em) {
            cols[n] = track;
            n += 1;
        }
        rest = rest.get(end..).unwrap_or("");
    }
    if n == 0 {
        return None;
    }
    Some((cols, n as u8))
}
