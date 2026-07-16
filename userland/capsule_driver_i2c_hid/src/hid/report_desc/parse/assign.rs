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

use crate::hid::report_desc::layout::{Field, TouchLayout};

const USAGE_PAGE_GENERIC_DESKTOP: u16 = 0x01;
const USAGE_PAGE_BUTTON: u16 = 0x09;
const USAGE_PAGE_DIGITIZER: u16 = 0x0D;

const USAGE_X: u16 = 0x30;
const USAGE_Y: u16 = 0x31;
const USAGE_TIP_SWITCH: u16 = 0x42;
const USAGE_CONFIDENCE: u16 = 0x47;
const USAGE_CONTACT_COUNT: u16 = 0x54;

// Record a field the first time its usage is seen, so multitouch reports that
// repeat X/Y per finger keep the primary contact.
pub(super) fn assign(
    layout: &mut TouchLayout,
    page: u16,
    usage: u16,
    report_id: u8,
    bit_offset: u32,
    bit_size: u32,
    logical_max: i32,
) {
    let field = Field { bit_offset, bit_size, logical_max };
    match (page, usage) {
        (USAGE_PAGE_GENERIC_DESKTOP, USAGE_X) if !layout.x.present() => {
            layout.x = field;
            layout.report_id = report_id;
        }
        (USAGE_PAGE_GENERIC_DESKTOP, USAGE_Y) if !layout.y.present() => layout.y = field,
        (USAGE_PAGE_DIGITIZER, USAGE_TIP_SWITCH) if !layout.tip.present() => layout.tip = field,
        (USAGE_PAGE_DIGITIZER, USAGE_CONFIDENCE) if !layout.confidence.present() => {
            layout.confidence = field
        }
        (USAGE_PAGE_DIGITIZER, USAGE_CONTACT_COUNT) if !layout.contact_count.present() => {
            layout.contact_count = field
        }
        (USAGE_PAGE_BUTTON, _) if !layout.button.present() => layout.button = field,
        _ => {}
    }
}
