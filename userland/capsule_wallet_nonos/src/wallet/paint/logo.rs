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

// Real brand mark, from the shared icon source, rasterized from wallet_logos/
// nonos-icon-teal.svg at 128x139. Rendered anti-aliased at any size, so it
// stays crisp instead of the old hand-traced 1-bit bitmap.
const ICON: &[u8] = include_bytes!("../../../../assets/icons/nonos_logo.rgba");
const ICON_W: u32 = 128;
const ICON_H: u32 = 139;

pub fn logo(fb: &mut PaintBuffer, x: u32, y: u32, s: u32) {
    if s == 0 {
        return;
    }
    // Preserve the mark's 128:139 aspect inside an s-tall box, centred.
    let dh = s;
    let dw = (s * ICON_W / ICON_H).max(1);
    let dx = x + s.saturating_sub(dw) / 2;
    fb.blit_rgba8_scaled(dx, y, dw, dh, ICON, ICON_W, ICON_H);
}
