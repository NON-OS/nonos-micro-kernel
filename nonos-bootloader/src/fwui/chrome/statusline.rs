// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::fwui::data::Sys;
use crate::fwui::draw::{fill_rect, hline};
use crate::fwui::layout::Layout;
use crate::fwui::metrics::{advance, line};
use crate::fwui::text::{text, text_right};
use crate::fwui::theme;
use crate::fwui::widget::status_dot;
use alloc::format;

pub fn statusline(lay: &Layout, sys: &Sys, desc: &[u8]) {
    let s = &lay.status;
    fill_rect(s.x, s.y.saturating_sub(line()), s.w, s.h + line(), theme::BG);
    hline(s.x, s.y.saturating_sub(line() / 2), s.w, theme::FRAME);
    text(s.x, s.y, desc, theme::DIM);
    let y2 = s.y + line();
    let on = |b: bool| if b { theme::OK } else { theme::MUTE };
    let adv = advance();
    let mut x = s.x;
    status_dot(x, y2, on(sys.secure_boot), b"SECURE BOOT", theme::DIM);
    x += adv * 16;
    status_dot(x, y2, on(sys.measured_boot), b"TPM", theme::DIM);
    x += adv * 8;
    status_dot(x, y2, on(sys.rng), b"RNG", theme::DIM);
    x += adv * 8;
    status_dot(x, y2, on(sys.ed25519 && sys.blake3), b"CRYPTO", theme::DIM);
    let m = format!("MEM {}MB   CPU {}   PCI {}", sys.mem_mib(), sys.cpu_count, sys.pci);
    text_right(s.right(), y2, m.as_bytes(), theme::DIM);
}
