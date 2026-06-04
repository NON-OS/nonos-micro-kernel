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
use crate::protocol::{E_INVAL, E_SHORT, E_SURFACE, STATUS_OK};

use super::attached_surface::attached_surface;
use super::constants::HEADER_LEN;
use crate::component_dispatch::kind::ComponentKind;
use crate::component_dispatch::paint::paint;

pub fn render(payload: &[u8]) -> u16 {
    if payload.len() < HEADER_LEN {
        return E_SHORT;
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    let x = u32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]);
    let y = u32::from_le_bytes([payload[12], payload[13], payload[14], payload[15]]);
    let w = u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]);
    let h = u32::from_le_bytes([payload[20], payload[21], payload[22], payload[23]]);
    let kind_raw = u16::from_le_bytes([payload[24], payload[25]]);
    let label_len = u16::from_le_bytes([payload[26], payload[27]]) as usize;
    if w == 0 || h == 0 || handle == 0 {
        return E_INVAL;
    }
    let kind = ComponentKind::from_raw(kind_raw);
    let label_end = HEADER_LEN.saturating_add(label_len);
    if label_end > payload.len() {
        return E_SHORT;
    }
    let label = &payload[HEADER_LEN..label_end];
    let Some(desc) = attached_surface(handle) else {
        return E_SURFACE;
    };
    paint(&desc, x, y, w, h, kind, label);
    STATUS_OK
}
