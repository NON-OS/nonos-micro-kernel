// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Grid geometry for the Launchpad: a centred, row-major grid holding every
//! desktop app, then every installed tool, then every capsule-store app.

use crate::state::{LAUNCHER_APPS, TOOL_APPS};

pub(super) const TILE: u32 = 72;
pub(super) const CELL_W: u32 = 120;
pub(super) const CELL_H: u32 = 108;
pub(super) const TOP_PAD: u32 = 110;

/// Total tiles: the built-in apps, then the command-line tools, then the
/// `installed` capsule-store apps the installer reported this boot, then the
/// `packages` installable files waiting in /pkgs.
pub(super) fn count(installed: usize, packages: usize) -> usize {
    LAUNCHER_APPS.len() + TOOL_APPS.len() + installed + packages
}

/// Columns that fit the display, kept within a sensible range so the grid stays
/// centred rather than stretching edge to edge on a wide screen.
pub(super) fn cols(width: u32) -> u32 {
    (width / CELL_W).clamp(1, 8)
}

/// Top-left screen position of the nth cell.
pub(super) fn cell_origin(width: u32, index: usize) -> (u32, u32) {
    let c = cols(width);
    let i = index as u32;
    let grid_w = c * CELL_W;
    let left = width.saturating_sub(grid_w) / 2;
    (left + (i % c) * CELL_W, TOP_PAD + (i / c) * CELL_H)
}
