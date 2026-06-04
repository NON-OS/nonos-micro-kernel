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

use crate::geometry::{clamp_to_display, Rect};
use crate::protocol::WINDOW_OPEN_REQ_LEN;
use crate::server::handlers::u32_at::u32_at;
use crate::window::{kind_from_u32, Kind};

pub(super) fn decode(body: &[u8], display_w: u32, display_h: u32) -> Option<(u32, Kind, Rect)> {
    if body.len() != WINDOW_OPEN_REQ_LEN {
        return None;
    }
    let window_id = u32_at(body, 0)?;
    let kind_raw = u32_at(body, 4)?;
    let x = u32_at(body, 8)?;
    let y = u32_at(body, 12)?;
    let w = u32_at(body, 16)?;
    let h = u32_at(body, 20)?;
    let kind = kind_from_u32(kind_raw)?;
    if window_id == 0 || w == 0 || h == 0 {
        return None;
    }
    Some((
        window_id,
        kind,
        clamp_to_display(Rect { x, y, width: w, height: h }, display_w, display_h),
    ))
}
