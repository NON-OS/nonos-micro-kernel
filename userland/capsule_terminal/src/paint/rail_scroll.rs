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
use super::rail_geom::telemetry_h;
use super::rail_left_geom::nav_h;
use super::rail_text::RAIL_GAP;
use crate::layout::rows::scroll_clamp;

/// Everything the column's height depends on. The painter and the hit-test both
/// build one of these from the same state, so both resolve the same offset.
#[derive(Clone, Copy)]
pub struct RailFit {
    pub sessions: u32,
    pub projects: u32,
    pub telemetry: bool,
}

pub fn content_h(f: RailFit) -> u32 {
    let nav = nav_h(f.sessions, f.projects);
    if !f.telemetry {
        return nav;
    }
    nav + RAIL_GAP + telemetry_h()
}

pub fn clamp(offset: u32, f: RailFit, viewport: u32) -> u32 {
    scroll_clamp(offset, content_h(f), viewport)
}

/// Rail-local top of the telemetry block, given an offset already clamped by
/// `clamp`. Both the painter and the hit-test clamp once and then share the
/// result, so neither can resolve a band the other never placed.
pub fn telemetry_top(off: u32, f: RailFit) -> i32 {
    (nav_h(f.sessions, f.projects) + RAIL_GAP) as i32 - off as i32
}
