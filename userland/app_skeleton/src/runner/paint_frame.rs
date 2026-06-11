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

use nonos_toolkit::decorations::{draw_border, draw_titlebar, DecorationHit};

use crate::app::{App, AppManifest};
use crate::clients::toolkit;
use crate::paint::PaintBuffer;
use crate::setup::WindowBinding;

use super::paint_close_button::paint_close_button;
use super::paint_maximize_button::paint_maximize_button;
use super::paint_minimize_button::paint_minimize_button;

const TITLEBAR_FILL_H: u32 = 28;
const TITLEBAR_FILL_ARGB: u32 = 0xFF1A_2030;
const TITLE_TEXT_ARGB: u32 = 0xFFD8_DEE8;
const BORDER_ARGB: u32 = 0xFF55_6677;

pub(super) fn paint<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &WindowBinding,
    toolkit_port: u32,
    request_id: u32,
    hover: DecorationHit,
) {
    let _ = toolkit::ui_frame(
        toolkit_port,
        request_id,
        binding.surface_handle,
        binding.width,
        binding.height,
    );
    let words = (binding.byte_len / 4) as usize;
    let pixels: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(binding.backing_va as *mut u32, words) };
    {
        let mut fb = PaintBuffer {
            pixels,
            stride_words: binding.stride_words,
            width: binding.width,
            height: binding.height,
        };
        app.paint(&mut fb);
        fb.fill_rect(0, 0, binding.width, TITLEBAR_FILL_H, TITLEBAR_FILL_ARGB);
    }
    let stride = binding.stride_words as usize;
    draw_titlebar(
        pixels,
        stride,
        binding.width,
        binding.height,
        TITLEBAR_FILL_ARGB,
        TITLE_TEXT_ARGB,
        manifest.title,
    );
    paint_close_button(pixels, stride, binding.width, hover == DecorationHit::CloseButton);
    paint_minimize_button(pixels, stride, binding.width, hover == DecorationHit::MinimizeButton);
    paint_maximize_button(pixels, stride, binding.width, hover == DecorationHit::MaximizeButton);
    draw_border(pixels, stride, binding.width, binding.height, BORDER_ARGB);
}
