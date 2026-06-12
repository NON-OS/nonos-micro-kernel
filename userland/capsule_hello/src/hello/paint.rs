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

const BG: u32 = 0xFF10161C;
const ACCENT: u32 = 0xFF00D4AA;
const TEXT: u32 = 0xFFE8F0F8;
const DIM: u32 = 0xFF667788;

pub fn paint(fb: &mut PaintBuffer) {
    fb.clear(BG);
    fb.fill_rect(0, 0, 360, 4, ACCENT);
    fb.text_scaled(24, 40, b"hello, NONOS", ACCENT, 2);
    fb.text(24, 88, b"a signed, attested capsule", TEXT);
    fb.text(24, 110, b"built from QUICKSTART.md", TEXT);
    fb.text(24, 148, b"press Esc to close", DIM);
}
