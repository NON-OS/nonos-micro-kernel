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

use crate::browser::css::computed::GridTrack;

use super::parse_px::parse_px;
use super::strip_unit::strip_unit;

// One grid track token. auto and the content keywords act as one fraction;
// minmax(a, b) collapses to its max side. The self-call terminates because
// each strip shortens the token.
pub(super) fn one_track(tok: &str, em: u32) -> Option<GridTrack> {
    let t = tok.trim();
    if t.eq_ignore_ascii_case("auto")
        || t.eq_ignore_ascii_case("min-content")
        || t.eq_ignore_ascii_case("max-content")
    {
        return Some(GridTrack::Fr(1));
    }
    if t.len() >= 8 && t[..7].eq_ignore_ascii_case("minmax(") {
        let inner = t.get(7..t.len() - 1)?;
        let comma = inner.rfind(',')?;
        return one_track(inner.get(comma + 1..)?, em);
    }
    if let Some(num) = strip_unit(t, "fr") {
        let f = num.trim().parse::<f32>().ok()?;
        if f.is_finite() && (0.0..=100.0).contains(&f) {
            return Some(GridTrack::Fr(((f + 0.5) as u16).max(1)));
        }
        return None;
    }
    if let Some(num) = t.strip_suffix('%') {
        let f = num.trim().parse::<f32>().ok()?;
        if f.is_finite() && (0.0..=100.0).contains(&f) {
            return Some(GridTrack::Pct((f + 0.5) as u16));
        }
        return None;
    }
    parse_px(t, em).map(GridTrack::Px)
}
