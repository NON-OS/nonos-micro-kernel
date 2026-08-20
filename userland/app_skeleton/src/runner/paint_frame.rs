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

use nonos_toolkit::decorations::{content_rect, draw_frame, DecorationHit};

use crate::app::{App, AppManifest};
use crate::clients::toolkit;
use crate::paint::PaintBuffer;
use crate::setup::WindowBinding;

use super::frame_finish::finish;

pub(super) fn paint<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &WindowBinding,
    toolkit_port: u32,
    request_id: u32,
    hover: DecorationHit,
    maximized: bool,
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
    let mut fb = PaintBuffer {
        pixels,
        stride_words: binding.stride_words,
        width: binding.width,
        height: binding.height,
    };
    let lit = hover != DecorationHit::None && hover != DecorationHit::Titlebar;
    draw_frame(&mut fb, maximized, manifest.title, lit);
    let c = content_rect(binding.width, binding.height, maximized);
    app.paint(&mut fb.sub(c.x, c.y, c.w, c.h));
    finish(&mut fb, maximized);
}
