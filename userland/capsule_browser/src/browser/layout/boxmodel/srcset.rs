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

// Pick one candidate from a srcset list. A candidate is "url [Nw|Nx]"; a
// bare url counts as 1x. Width descriptors rank directly, density as
// `want * N`. Formats the decoder cannot handle are skipped whenever a
// decodable candidate exists. Preference: the narrowest candidate at or
// above `want`, else the widest available.
pub(super) fn pick_srcset(list: &str, want: i32) -> Option<String> {
    let mut best_fit: Option<(i32, &str)> = None;
    let mut widest: Option<(i32, &str)> = None;
    let mut widest_any: Option<(i32, &str)> = None;
    for cand in list.split(',') {
        let mut it = cand.split_whitespace();
        let Some(url) = it.next() else { continue };
        if url.is_empty() {
            continue;
        }
        let w = match it.next() {
            Some(d) if d.ends_with('w') => {
                d[..d.len() - 1].parse::<i32>().unwrap_or(0).clamp(1, 65_536)
            }
            Some(d) if d.ends_with('x') => {
                let f = d[..d.len() - 1].parse::<f32>().unwrap_or(1.0);
                ((want as f32 * f) as i32).clamp(1, 65_536)
            }
            _ => want,
        };
        if widest_any.is_none_or(|(bw, _)| w > bw) {
            widest_any = Some((w, url));
        }
        if !decodable(url) {
            continue;
        }
        if w >= want && best_fit.is_none_or(|(bw, _)| w < bw) {
            best_fit = Some((w, url));
        }
        if widest.is_none_or(|(bw, _)| w > bw) {
            widest = Some((w, url));
        }
    }
    best_fit.or(widest).or(widest_any).map(|(_, u)| u.to_string())
}

// The decoder handles PNG/JPEG/BMP/GIF/SVG; skip formats it would reject so
// a fallback candidate gets the slot instead.
fn decodable(url: &str) -> bool {
    let path = url.split(['?', '#']).next().unwrap_or(url).to_ascii_lowercase();
    !(path.ends_with(".webp") || path.ends_with(".avif"))
}
