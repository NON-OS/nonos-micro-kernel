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

use nonos_app_skeleton::PaintBuffer;

use super::rail_band::Band;
use super::rail_left_geom::Sections;
use super::rail_text::{band_rect, head};
use crate::term::theme::types::Theme;

/// The two list captions and the affordance that adds to each. Drawn from the
/// same `Sections` the hit-test reads, so the cross a click lands on is the
/// cross the frame put there.
pub fn caption(fb: &mut PaintBuffer, s: &Sections, t: &Theme) {
    head(fb, s.s_head.x, s.s_head.y, s.s_head.w, "SESSIONS", t);
    plus(fb, &s.s_plus, t);
    head(fb, s.p_head.x, s.p_head.y, s.p_head.w, "PROJECTS", t);
    plus(fb, &s.p_plus, t);
}

fn plus(fb: &mut PaintBuffer, r: &Band, t: &Theme) {
    let arm = r.w / 2;
    let cx = r.x + r.w / 2;
    let cy = r.y + (r.h / 2) as i32;
    band_rect(fb, cx.saturating_sub(arm / 2), cy, arm, 1, t.dim);
    band_rect(fb, cx, cy - (arm / 2) as i32, 1, arm, t.dim);
}
