// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Real artwork for installed tools. Each shipped tool has a 48x48 glyph mask
//! under assets/app_icons/tools, drawn in the same cyan line-art language as
//! the desktop apps. A tool installed without artwork falls back to the
//! generated tile, so a missing icon never blocks an install.

use super::gen_icon;
use crate::render::draw_tool_icon;
use crate::state::Context;

const GREX: &[u8] = include_bytes!("../../../assets/app_icons/tools/grex.rgba");
const DOTENV_LINTER: &[u8] = include_bytes!("../../../assets/app_icons/tools/dotenv_linter.rgba");
const PASTEL: &[u8] = include_bytes!("../../../assets/app_icons/tools/pastel.rgba");
const JSONXF: &[u8] = include_bytes!("../../../assets/app_icons/tools/jsonxf.rgba");
const CHOOSE: &[u8] = include_bytes!("../../../assets/app_icons/tools/choose.rgba");
const TOKEI: &[u8] = include_bytes!("../../../assets/app_icons/tools/tokei.rgba");
const HUNIQ: &[u8] = include_bytes!("../../../assets/app_icons/tools/huniq.rgba");
const CSVIEW: &[u8] = include_bytes!("../../../assets/app_icons/tools/csview.rgba");

fn glyph_for(label: &[u8]) -> Option<&'static [u8]> {
    match label {
        b"grex" => Some(GREX),
        b"dotenv-linter" => Some(DOTENV_LINTER),
        b"pastel" => Some(PASTEL),
        b"jsonxf" => Some(JSONXF),
        b"choose" => Some(CHOOSE),
        b"tokei" => Some(TOKEI),
        b"huniq" => Some(HUNIQ),
        b"csview" => Some(CSVIEW),
        _ => None,
    }
}

pub(super) fn draw(ctx: &Context, x: u32, y: u32, size: u32, label: &[u8]) {
    match glyph_for(label) {
        Some(glyph) => draw_tool_icon(ctx, x, y, size, glyph),
        None => gen_icon::draw(ctx, x, y, size, label),
    }
}
