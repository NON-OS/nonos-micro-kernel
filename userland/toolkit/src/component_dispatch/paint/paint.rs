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
use nonos_libc::SurfaceDescriptor;

use crate::components::button::{render_button, ButtonStyle};
use crate::components::label::{render_label, LabelStyle};
use crate::design::color::Argb;
use crate::theme;

use super::fill_rect::fill_rect;
use crate::component_dispatch::kind::ComponentKind;

pub fn paint(
    desc: &SurfaceDescriptor,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    kind: ComponentKind,
    label: &[u8],
) {
    let stride_px = (desc.stride as usize) / 4;
    let total = stride_px.saturating_mul(desc.height as usize);
    let buf = unsafe { core::slice::from_raw_parts_mut(desc.base_va as *mut u32, total) };
    let t = theme::snapshot();
    match kind {
        ComponentKind::Panel => {
            fill_rect(buf, stride_px, desc.width, desc.height, (x, y, w, h), t.surface_argb);
        }
        ComponentKind::Button => {
            let style = ButtonStyle { bg: Argb(t.accent_argb), fg: Argb(t.text_argb) };
            render_button(buf, stride_px, desc.width, desc.height, x, y, w, h, label, style);
        }
        ComponentKind::Label => {
            let style = LabelStyle { color: Argb(t.text_argb) };
            render_label(buf, stride_px, desc.width, desc.height, x, y, label, style);
        }
    }
}
