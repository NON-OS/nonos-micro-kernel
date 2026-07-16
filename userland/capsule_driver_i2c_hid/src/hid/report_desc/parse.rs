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

//! Walk a HID report descriptor and locate the touchpad's absolute X/Y, tip
//! switch, contact count and button fields. This is a focused reader: it tracks
//! only the global and local state those fields need, records the first contact
//! it sees (enough to drive the cursor), and is bounded so a malformed
//! descriptor can only ever yield fewer fields, never loop or read out of range.

use super::layout::{Field, TouchLayout};

const USAGE_PAGE_GENERIC_DESKTOP: u16 = 0x01;
const USAGE_PAGE_BUTTON: u16 = 0x09;
const USAGE_PAGE_DIGITIZER: u16 = 0x0D;

const USAGE_X: u16 = 0x30;
const USAGE_Y: u16 = 0x31;
const USAGE_TIP_SWITCH: u16 = 0x42;
const USAGE_CONFIDENCE: u16 = 0x47;
const USAGE_CONTACT_COUNT: u16 = 0x54;
const USAGE_INPUT_MODE: u16 = 0x52;
const USAGE_SURFACE_SWITCH: u16 = 0x57;
const USAGE_BUTTON_SWITCH: u16 = 0x58;

const MAX_USAGES: usize = 64;
const MAX_FIELDS_PER_ITEM: u32 = 256;

pub fn parse(desc: &[u8]) -> TouchLayout {
    // A precision touchpad's descriptor declares the vestigial mouse
    // collection first (relative X/Y, no tip switch) and the absolute touch
    // collection after it. Only the touch collection carries a Digitizer Tip
    // Switch, so find that report id first and take every input field from it
    // alone; otherwise the parser locks onto the mouse collection's X and the
    // whole touch layout collapses to the relative boot-mouse path — random,
    // fast, click-spraying motion. Zero means the device uses no report ids
    // (single report), in which case every field is in that one report.
    let touch_rid = find_touch_report_id(desc);
    parse_with_rid(desc, touch_rid)
}

// The report id of the input report that declares a Digitizer Tip Switch.
// Zero when none is found (also the no-report-id case).
fn find_touch_report_id(desc: &[u8]) -> u8 {
    let mut usage_page: u16 = 0;
    let mut report_id: u8 = 0;
    let mut report_count: u32 = 0;
    let mut usages = [0u16; MAX_USAGES];
    let mut n_usages = 0usize;

    let mut i = 0;
    while i < desc.len() {
        let b0 = desc[i];
        i += 1;
        if b0 == 0xFE {
            let Some(&dsize) = desc.get(i) else { break };
            i = i.saturating_add(2 + dsize as usize);
            continue;
        }
        let size = item_size(b0);
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
                0x8 => report_id = data as u8,
                0x9 => report_count = data,
                _ => {}
            },
            2 => {
                if tag == 0x0 && n_usages < MAX_USAGES {
                    usages[n_usages] = data as u16;
                    n_usages += 1;
                }
            }
            0 => {
                if tag == 0x8 {
                    let count = report_count.min(MAX_FIELDS_PER_ITEM);
                    for f in 0..count {
                        let usage = usage_for(&usages, n_usages, f as usize);
                        if usage_page == USAGE_PAGE_DIGITIZER && usage == USAGE_TIP_SWITCH {
                            return report_id;
                        }
                    }
                }
                n_usages = 0;
            }
            _ => {}
        }
    }
    0
}

fn parse_with_rid(desc: &[u8], touch_rid: u8) -> TouchLayout {
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
        let size = item_size(b0);
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
                        // Input fields count only when they live in the touch
                        // report: a mouse-collection X at the same usage would
                        // otherwise be recorded and drag the whole layout onto
                        // the wrong, relative report.
                        if report_id == touch_rid {
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
                        }
                        bit_offset = bit_offset.saturating_add(report_size);
                    }
                } else if tag == 0xB {
                    let count = report_count.min(MAX_FIELDS_PER_ITEM);
                    for f in 0..count {
                        let usage = usage_for(&usages, n_usages, f as usize);
                        if usage_page == USAGE_PAGE_DIGITIZER {
                            let field = Field {
                                bit_offset: feature_bit_offset,
                                bit_size: report_size,
                                logical_max,
                            };
                            match usage {
                                USAGE_INPUT_MODE if !layout.input_mode.present() => {
                                    layout.input_mode = field;
                                    layout.input_mode_report_id = report_id;
                                }
                                USAGE_SURFACE_SWITCH if !layout.surface_switch.present() => {
                                    layout.surface_switch = field;
                                    layout.surface_switch_report_id = report_id;
                                }
                                USAGE_BUTTON_SWITCH if !layout.button_switch.present() => {
                                    layout.button_switch = field;
                                    layout.button_switch_report_id = report_id;
                                }
                                _ => {}
                            }
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

// Data-bytes size encoded in a short item's low two bits.
fn item_size(b0: u8) -> usize {
    match b0 & 0x03 {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 4,
    }
}

// The nth field's usage; when a report declares more fields than usages, the
// last usage applies to the remainder (per the HID spec).
fn usage_for(usages: &[u16], n: usize, index: usize) -> u16 {
    if n == 0 {
        0
    } else if index < n {
        usages[index]
    } else {
        usages[n - 1]
    }
}

// Record a field the first time its usage is seen, so multitouch reports that
// repeat X/Y per finger keep the primary contact.
fn assign(
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

fn read_le(bytes: &[u8]) -> u32 {
    let mut v = 0u32;
    for (k, &b) in bytes.iter().enumerate() {
        v |= (b as u32) << (8 * k as u32);
    }
    v
}
