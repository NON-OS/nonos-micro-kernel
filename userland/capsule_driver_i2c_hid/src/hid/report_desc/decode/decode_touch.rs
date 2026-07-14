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

use super::types::TouchSample;
use crate::hid::report_desc::layout::TouchLayout;
use crate::hid::report_desc::read_bits::read_bits;

/// `report` is the input report after the two-byte i2c-hid length prefix (what
/// the poll loop already slices out). Returns None when the report is for a
/// different report id than the touch layout.
pub fn decode_touch(report: &[u8], layout: &TouchLayout) -> Option<TouchSample> {
    let body = if layout.report_id != 0 {
        if report.first().copied()? != layout.report_id {
            return None;
        }
        &report[1..]
    } else {
        report
    };

    // A torn or short polled read must be dropped whole: read_bits zero-fills
    // past the end, which would silently truncate coordinates into violent
    // jumps toward the origin instead of failing.
    let fields = [layout.x, layout.y, layout.tip, layout.contact_count, layout.button];
    let need =
        fields.iter().filter(|f| f.present()).map(|f| f.bit_offset + f.bit_size).max().unwrap_or(0);
    if (body.len() as u32) * 8 < need {
        return None;
    }

    let x = read_bits(body, layout.x.bit_offset, layout.x.bit_size);
    let y = read_bits(body, layout.y.bit_offset, layout.y.bit_size);
    let tip = read_bits(body, layout.tip.bit_offset, layout.tip.bit_size) != 0;
    let contacts = if layout.contact_count.present() {
        read_bits(body, layout.contact_count.bit_offset, layout.contact_count.bit_size)
    } else if tip {
        1
    } else {
        0
    };
    let button = layout.button.present()
        && read_bits(body, layout.button.bit_offset, layout.button.bit_size) != 0;

    Some(TouchSample {
        x,
        y,
        x_max: layout.x.logical_max,
        y_max: layout.y.logical_max,
        tip,
        contacts,
        button,
    })
}
