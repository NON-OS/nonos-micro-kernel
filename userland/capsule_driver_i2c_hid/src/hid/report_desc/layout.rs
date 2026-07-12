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
}

impl TouchLayout {
    pub fn is_absolute_touch(&self) -> bool {
        self.x.present() && self.y.present() && self.tip.present()
    }
}
