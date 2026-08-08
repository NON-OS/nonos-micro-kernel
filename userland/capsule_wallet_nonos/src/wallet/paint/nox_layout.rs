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

/// Where every control on the staking screen sits, for a given width.
///
/// One computation, read by both the painter and the hit-testing. They used
/// to carry the same numbers separately, which is why a resized window drew
/// the controls in one place and answered clicks in another. Whatever this
/// says, both sides agree.
pub struct NoxLayout {
    pub cx: u32,
    pub cw: u32,
    /// Left card holding the amount and the action.
    pub lw: u32,
    /// Right column; equal to the left card when the two stack.
    pub rx: u32,
    pub rw: u32,
    /// True when the width cannot hold two columns and the right one drops
    /// under the left.
    pub stacked: bool,
    pub tab_w: u32,
    pub track_x: u32,
    pub track_w: u32,
    /// Viewport height, or zero when it is not known.
    pub height: u32,
}

/// Side margin the screens share, matching paint_nox.rs.
const MARGIN: u32 = 226;
/// Inset kept on the right, also matching the rest of the screen.
const RIGHT_INSET: u32 = 26;
/// Narrower than this and two columns stop being readable.
const MIN_RIGHT: u32 = 260;
/// The left card never shrinks past the point its controls stop fitting.
const MIN_LEFT: u32 = 320;

impl NoxLayout {
    pub fn new(width: u32) -> Self {
        Self::sized(width, 0)
    }

    /// Layout for a viewport of this width and height. Height of zero means
    /// unknown, which keeps the roomy arrangement.
    pub fn sized(width: u32, height: u32) -> Self {
        // The same inset the rest of the NOX screen uses, so the staking card
        // lines up with the summary cards above it instead of hanging into
        // the sidebar. Only a window too narrow to hold that gives it up.
        let cx = if width > MARGIN + RIGHT_INSET + MIN_LEFT { MARGIN } else { width / 8 };
        let cw = width.saturating_sub(cx + RIGHT_INSET).max(MIN_LEFT);
        // Two columns only while both can be themselves. Below that the right
        // column goes under the left at full width rather than being squeezed
        // into a sliver, or worse, given a width that underflowed.
        let stacked = cw < MIN_LEFT + 16 + MIN_RIGHT;
        let lw = if stacked { cw } else { (cw * 5 / 8).max(MIN_LEFT) };
        let (rx, rw) = if stacked { (cx, cw) } else { (cx + lw + 16, cw.saturating_sub(lw + 16)) };
        let tab_w = (lw.saturating_sub(40)) / 2;
        Self {
            height,
            cx,
            cw,
            lw,
            rx,
            rw,
            stacked,
            tab_w,
            track_x: cx + 20,
            track_w: lw.saturating_sub(40),
        }
    }
}
