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

//! The fields a touchpad input report exposes, located by parsing the HID
//! report descriptor. Offsets are in bits from the start of the report body
//! (after the report-id byte, if any).

#[derive(Clone, Copy, Default)]
pub struct Field {
    pub bit_offset: u32,
    pub bit_size: u32,
    pub logical_max: i32,
}

impl Field {
    pub fn present(&self) -> bool {
        self.bit_size != 0
    }
}

/// The located fields of a precision-touchpad report. A layout is only usable
/// when it carries absolute X and Y and a tip switch; otherwise the driver
/// falls back to the relative boot-mouse decode.
#[derive(Clone, Copy, Default)]
pub struct TouchLayout {
    /// Report id that carries the touch data, or 0 when the device uses none.
    pub report_id: u8,
    pub x: Field,
    pub y: Field,
    pub tip: Field,
    pub contact_count: Field,
    pub button: Field,
    /// The PTP Confidence bit (usage 0x0D:0x47): set when the contact is a
    /// deliberate finger, clear for a palm or accidental contact. Absent on
    /// non-PTP devices; treated as always-confident then.
    pub confidence: Field,
    /// The Input Mode field of the device-configuration FEATURE report (usage
    /// 0x0D:0x52), with its bit offset inside that feature report. A precision
    /// touchpad powers up in mouse mode and only streams the touch collection
    /// after the host writes mode 3 here.
    pub input_mode: Field,
    /// Report id of the feature report carrying the Input Mode field.
    pub input_mode_report_id: u8,
    /// Surface-switch feature field (usage 0x0D:0x57): zero disables all
    /// surface (motion) reporting. A pad with this cleared looks dead.
    pub surface_switch: Field,
    pub surface_switch_report_id: u8,
    /// Button-switch feature field (usage 0x0D:0x58): zero disables button
    /// reporting.
    pub button_switch: Field,
    pub button_switch_report_id: u8,
}

impl TouchLayout {
    pub fn is_absolute_touch(&self) -> bool {
        self.x.present() && self.y.present() && self.tip.present()
    }
}
