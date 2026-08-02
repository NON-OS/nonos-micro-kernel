// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Which Launchpad tile a point falls on, and what it launches.

use super::grid::{cell_origin, count, CELL_H, CELL_W};
use crate::state::{LAUNCHER_APPS, TOOL_APPS};

/// What a clicked tile launches: a desktop app by index, an installed tool by
/// index into the tool table, or a capsule-store app by index into the
/// installed-app list the installer reported.
pub enum Target {
    App(usize),
    Tool(usize),
    Installed(usize),
}

/// Index of the tile under a point, if any. `installed` is how many store apps
/// the grid currently carries after the built-in sections.
pub fn hit(width: u32, px: u32, py: u32, installed: usize) -> Option<usize> {
    (0..count(installed)).find(|&i| {
        let (x, y) = cell_origin(width, i);
        px >= x && px < x + CELL_W && py >= y && py < y + CELL_H
    })
}

/// Resolve a tile index to the app, tool or store app it stands for. An index
/// past every section resolves to the last store slot rather than panicking a
/// caller that raced the list growing.
pub fn target(index: usize, installed: usize) -> Target {
    let apps = LAUNCHER_APPS.len();
    let tools = apps + TOOL_APPS.len();
    if index < apps {
        Target::App(index)
    } else if index < tools {
        Target::Tool(index - apps)
    } else {
        Target::Installed((index - tools).min(installed.saturating_sub(1)))
    }
}
