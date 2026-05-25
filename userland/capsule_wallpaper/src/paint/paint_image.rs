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

use crate::state::Context;

use super::blit_argb::blit_argb;
use super::decode_jpeg::DecodedImage;

pub fn paint_image(ctx: &Context, img: &DecodedImage) {
    blit_argb(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        &img.pixels,
        img.width,
        img.height,
    );
}
