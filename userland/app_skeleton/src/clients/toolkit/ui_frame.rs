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

use super::constants::{CHROME_H, KIND_LABEL, KIND_PANEL, LABEL_X, LABEL_Y};
use super::render_component::render_component;

pub fn ui_frame(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    width: u32,
    title: &[u8],
) -> Result<(), &'static str> {
    render_component(port, request_id, surface_handle, 0, 0, width, CHROME_H, KIND_PANEL, b"")?;
    render_component(
        port,
        request_id.wrapping_add(1),
        surface_handle,
        LABEL_X,
        LABEL_Y,
        width.saturating_sub(LABEL_X.saturating_mul(2)),
        14,
        KIND_LABEL,
        title,
    )
}
