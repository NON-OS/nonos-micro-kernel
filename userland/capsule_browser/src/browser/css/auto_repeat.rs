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

use crate::browser::css::computed::{AutoRepeat, GridTrack};

use super::matching_paren::matching_paren;
use super::one_track::one_track;

// A whole template of the form repeat(auto-fill | auto-fit, <track>), the
// shape every responsive card grid is written in. The repetition count needs
// the container width, so this returns the keyword, the track's floor and the
// track itself, and layout works out how many fit.
//
// Anything with a track beside the repeat() is left to parse_grid_tracks; the
// count would have to be resolved against the leftover space rather than the
// whole width, and this engine sizes tracks in one pass.
pub(super) fn parse_auto_repeat(
    value: &str,
    em: u32,
) -> Option<(AutoRepeat, GridTrack, GridTrack)> {
    let v = value.trim();
    if v.len() < 8 || !v[..7].eq_ignore_ascii_case("repeat(") {
        return None;
    }
    let close = matching_paren(&v[7..]);
    if 7 + close + 1 != v.len() {
        return None;
    }
    let body = v.get(7..7 + close)?;
    let comma = body.find(',')?;
    let head = body[..comma].trim();
    let mode = if head.eq_ignore_ascii_case("auto-fill") {
        AutoRepeat::Fill
    } else if head.eq_ignore_ascii_case("auto-fit") {
        AutoRepeat::Fit
    } else {
        return None;
    };
    let spec = body.get(comma + 1..)?.trim();
    let track = one_track(spec, em)?;
    Some((mode, min_track(spec, em).unwrap_or(track), track))
}

// The floor a track will not shrink below: the min side of minmax(), or the
// track itself when it is a plain length.
fn min_track(spec: &str, em: u32) -> Option<GridTrack> {
    let t = spec.trim();
    if t.len() >= 8 && t[..7].eq_ignore_ascii_case("minmax(") {
        let inner = t.get(7..t.len() - 1)?;
        let comma = inner.rfind(',')?;
        return one_track(inner.get(..comma)?, em);
    }
    one_track(t, em)
}
