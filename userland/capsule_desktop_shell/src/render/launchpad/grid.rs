// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Grid geometry for the Launchpad: a centred, row-major grid holding every
//! desktop app, then every installed tool, then every capsule-store app.

use crate::render::ui_font;
use crate::state::{LAUNCHER_APPS, TOOL_APPS};

const TILE_LOGICAL: u32 = 72;
const CELL_W_LOGICAL: u32 = 120;
const CELL_H_LOGICAL: u32 = 108;
const TOP_PAD_LOGICAL: u32 = 110;

pub(super) fn tile() -> u32 {
    TILE_LOGICAL * ui_font::scale()
}

pub(super) fn cell_w() -> u32 {
    CELL_W_LOGICAL * ui_font::scale()
}

pub(super) fn cell_h() -> u32 {
    CELL_H_LOGICAL * ui_font::scale()
}

pub(super) fn top_pad() -> u32 {
    TOP_PAD_LOGICAL * ui_font::scale()
}

/// Total tiles: the built-in apps, then the command-line tools, then the
/// `installed` capsule-store apps the installer reported this boot, then the
/// `packages` installable files waiting in /pkgs.
pub(super) fn count(installed: usize, packages: usize) -> usize {
    LAUNCHER_APPS.len() + TOOL_APPS.len() + installed + packages
}

/// Columns that fit the display, kept within a sensible range so the grid stays
/// centred rather than stretching edge to edge on a wide screen.
pub(super) fn cols(width: u32) -> u32 {
    (width / cell_w()).clamp(1, 8)
}

/// Top-left screen position of the nth cell.
pub(super) fn cell_origin(width: u32, index: usize) -> (u32, u32) {
    let c = cols(width);
    let i = index as u32;
    let grid_w = c * cell_w();
    let left = width.saturating_sub(grid_w) / 2;
    (left + (i % c) * cell_w(), top_pad() + (i / c) * cell_h())
}
