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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::matching_paren::matching_paren;
use super::one_track::one_track;

// Named column lines from a grid-template-columns value: each [a b] group
// names the line before the next track. Tokenization mirrors
// parse_grid_tracks so the recorded indices line up with the parsed tracks.
pub(super) fn col_line_names(value: &str, em: u32) -> Vec<(String, u8)> {
    let mut out: Vec<(String, u8)> = Vec::new();
    let mut track = 0u8;
    let mut rest = value.trim();
    while !rest.is_empty() && out.len() < 32 {
        rest = rest.trim_start();
        if let Some(after) = rest.strip_prefix('[') {
            let close = after.find(']').unwrap_or(after.len());
            for name in after[..close].split_whitespace() {
                out.push((name.to_string(), track));
            }
            rest = after.get(close + 1..).unwrap_or("");
            continue;
        }
        let low = rest.to_ascii_lowercase();
        let end = if low.starts_with("minmax(") || low.starts_with("repeat(") {
            (7 + matching_paren(&rest[7..]) + 1).min(rest.len())
        } else {
            rest.find(char::is_whitespace).unwrap_or(rest.len())
        };
        // repeat() expands to several tracks; named lines after it would
        // drift, so give up on the remainder rather than record bad indices.
        if low.starts_with("repeat(") {
            break;
        }
        if one_track(&rest[..end], em).is_some() {
            track = track.saturating_add(1);
        }
        rest = rest.get(end..).unwrap_or("");
    }
    out
}
