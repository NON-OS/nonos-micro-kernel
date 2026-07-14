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

use super::assign::assign;
use super::read_le::read_le;
use super::usage_for::usage_for;
use crate::hid::report_desc::layout::{Field, TouchLayout};

const USAGE_PAGE_DIGITIZER: u16 = 0x0D;
const USAGE_INPUT_MODE: u16 = 0x52;

const MAX_USAGES: usize = 64;
const MAX_FIELDS_PER_ITEM: u32 = 256;

pub fn parse(desc: &[u8]) -> TouchLayout {
    let mut layout = TouchLayout::default();
    let mut usage_page: u16 = 0;
    let mut report_size: u32 = 0;
    let mut report_count: u32 = 0;
    let mut report_id: u8 = 0;
    let mut logical_max: i32 = 0;
    let mut bit_offset: u32 = 0;
    // Feature reports lay out in their own offset space, separate from input
    // reports of the same id.
    let mut feature_bit_offset: u32 = 0;
    let mut usages = [0u16; MAX_USAGES];
    let mut n_usages = 0usize;

    let mut i = 0;
    while i < desc.len() {
        let b0 = desc[i];
        i += 1;
        if b0 == 0xFE {
            // Long item: [size][tag][data...]. We do not need any long item, so
            // step over it safely.
            let Some(&dsize) = desc.get(i) else { break };
            i = i.saturating_add(2 + dsize as usize);
            continue;
        }
        let size = match b0 & 0x03 {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => 4,
        };
        if i + size > desc.len() {
            break;
        }
        let data = read_le(&desc[i..i + size]);
        i += size;
        let btype = (b0 >> 2) & 0x03;
        let tag = (b0 >> 4) & 0x0F;

        match btype {
            1 => match tag {
                0x0 => usage_page = data as u16,
                0x2 => logical_max = data as i32,
                0x7 => report_size = data,
                0x8 => {
                    report_id = data as u8;
                    bit_offset = 0;
                    feature_bit_offset = 0;
                }
                0x9 => report_count = data,
                _ => {}
            },
            2 => {
                // Local Usage; ignore usage min/max ranges for this purpose.
                if tag == 0x0 && n_usages < MAX_USAGES {
                    usages[n_usages] = data as u16;
                    n_usages += 1;
                }
            }
            0 => {
                // Main item. Input (tag 0x8) lays out input-report fields;
                // Feature (tag 0xB) is walked only for the Input Mode switch.
                if tag == 0x8 {
                    let count = report_count.min(MAX_FIELDS_PER_ITEM);
                    for f in 0..count {
                        let usage = usage_for(&usages, n_usages, f as usize);
                        assign(
                            &mut layout,
                            usage_page,
                            usage,
                            report_id,
                            bit_offset,
                            report_size,
                            logical_max,
                        );
                        bit_offset = bit_offset.saturating_add(report_size);
                    }
                } else if tag == 0xB {
                    let count = report_count.min(MAX_FIELDS_PER_ITEM);
                    for f in 0..count {
                        let usage = usage_for(&usages, n_usages, f as usize);
                        if usage_page == USAGE_PAGE_DIGITIZER
                            && usage == USAGE_INPUT_MODE
                            && !layout.input_mode.present()
                        {
                            layout.input_mode = Field {
                                bit_offset: feature_bit_offset,
                                bit_size: report_size,
                                logical_max,
                            };
                            layout.input_mode_report_id = report_id;
                        }
                        feature_bit_offset = feature_bit_offset.saturating_add(report_size);
                    }
                }
                // Locals are cleared after every main item.
                n_usages = 0;
            }
            _ => {}
        }
    }
    layout
}
