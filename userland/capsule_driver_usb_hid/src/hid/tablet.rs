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

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_WHEEL,
};

use super::post_wire::{send, send_abs};

pub struct Tablet {
    buttons: u8,
}

impl Tablet {
    pub fn new() -> Self {
        Self { buttons: 0 }
    }

    pub fn feed(&mut self, report: &[u8]) {
        if report.len() < 5 {
            return;
        }
        let buttons = report[0] & 0x07;
        let x = u16::from_le_bytes([report[1], report[2]]) as i32;
        let y = u16::from_le_bytes([report[3], report[4]]) as i32;
        let dz = if report.len() > 5 { report[5] as i8 as i32 } else { 0 };
        let _ = send_abs(INPUT_KIND_POINTER_ABS, x, y);
        if dz != 0 {
            let _ = send(INPUT_KIND_WHEEL, 0, 0, 0, dz);
        }
        publish_buttons(self.buttons, buttons);
        self.buttons = buttons;
    }
}

fn publish_buttons(previous: u8, current: u8) {
    let changed = (previous ^ current) & 0x07;
    let mut bit = 0u8;
    while bit < 3 {
        let mask = 1u8 << bit;
        if changed & mask != 0 {
            let down = current & mask != 0;
            let kind = if down { INPUT_KIND_BUTTON_DOWN } else { INPUT_KIND_BUTTON_UP };
            let _ = send(kind, 0, u32::from(bit) + 1, 0, 0);
        }
        bit += 1;
    }
}
