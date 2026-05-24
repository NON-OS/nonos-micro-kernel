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

use super::row;
use crate::about::data::{caps, trust};
use crate::about::format::u64_decimal;

const FIXED_ROWS: u32 = 6;

pub fn line_count() -> u32 {
    let granted = caps::ALL_CAPS.iter().filter(|c| caps::is_granted(c.bit)).count() as u32;
    let denied = caps::ALL_CAPS.iter().filter(|c| !caps::is_granted(c.bit)).count() as u32;
    FIXED_ROWS + granted + 1 + denied
}

pub fn render(scroll: u32, visible: u32, top: u32, fb: &mut PaintBuffer) {
    let mut buf = [0u8; 20];
    let total = line_count();
    let end = scroll.saturating_add(visible).min(total);
    let mut idx: u32 = 0;
    let mut visible_idx: u32 = 0;
    let fixed: [(&[u8], &[u8]); 6] = [
        (b"Chain", trust::SIGNING_CHAIN),
        (b"Scheme", trust::HYBRID_SCHEME),
        (b"Manifest", trust::MANIFEST_FORMAT),
        (b"Cert", trust::CERT_FORMAT),
        (b"Status", trust::STATUS),
        (b"Cap mask", u64_decimal(caps::MASK, &mut buf)),
    ];
    for (label, value) in fixed {
        if idx >= scroll && idx < end {
            row::pair(label, value, row::line_y(visible_idx, top), fb);
            visible_idx += 1;
        }
        idx += 1;
    }
    for descriptor in caps::ALL_CAPS.iter().filter(|c| caps::is_granted(c.bit)) {
        if idx >= scroll && idx < end {
            row::pair(descriptor.name, descriptor.role, row::line_y(visible_idx, top), fb);
            visible_idx += 1;
        }
        idx += 1;
    }
    if idx >= scroll && idx < end {
        row::single(b"Denied:", row::line_y(visible_idx, top), fb);
        visible_idx += 1;
    }
    idx += 1;
    for descriptor in caps::ALL_CAPS.iter().filter(|c| !caps::is_granted(c.bit)) {
        if idx >= scroll && idx < end {
            row::single(descriptor.name, row::line_y(visible_idx, top), fb);
            visible_idx += 1;
        }
        idx += 1;
    }
}
