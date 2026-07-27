// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Which Launchpad tile a point falls on, and what it launches.

use super::grid::{cell_origin, count, CELL_H, CELL_W};
use crate::state::LAUNCHER_APPS;

/// What a clicked tile launches: a desktop app by index, or an installed tool
/// by index into the tool table.
pub enum Target {
    App(usize),
    Tool(usize),
}

/// Index of the tile under a point, if any.
pub fn hit(width: u32, px: u32, py: u32) -> Option<usize> {
    (0..count()).find(|&i| {
        let (x, y) = cell_origin(width, i);
        px >= x && px < x + CELL_W && py >= y && py < y + CELL_H
    })
}

/// Resolve a tile index to the app or tool it stands for.
pub fn target(index: usize) -> Target {
    let apps = LAUNCHER_APPS.len();
    if index < apps {
        Target::App(index)
    } else {
        Target::Tool(index - apps)
    }
}
