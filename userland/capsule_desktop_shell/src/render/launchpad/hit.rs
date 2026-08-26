// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Which Launchpad tile a point falls on, and what it launches.

use super::grid::{cell_h, cell_origin, cell_w, count};
use crate::state::{LAUNCHER_APPS, TOOL_APPS};

/// What a clicked tile launches: a desktop app by index, an installed tool by
/// index into the tool table, a capsule-store app by index into the
/// installed-app list the installer reported, or a not-yet-installed package
/// by index into the /pkgs scan.
#[derive(Clone, Copy)]
pub enum Target {
    App(usize),
    Tool(usize),
    Installed(usize),
    Package(usize),
}

/// Index of the tile under a point, if any. `installed` is how many store apps
/// the grid currently carries after the built-in sections, `packages` how many
/// installable files follow them.
pub fn hit(width: u32, px: u32, py: u32, installed: usize, packages: usize) -> Option<usize> {
    (0..count(installed, packages)).find(|&i| {
        let (x, y) = cell_origin(width, i);
        px >= x && px < x + cell_w() && py >= y && py < y + cell_h()
    })
}

/// Resolve a tile index to the app, tool, store app or package it stands for.
/// An index past every section resolves to the last package slot rather than
/// panicking a caller that raced the list growing.
pub fn target(index: usize, installed: usize, packages: usize) -> Target {
    let apps = LAUNCHER_APPS.len();
    let tools = apps + TOOL_APPS.len();
    let stored = tools + installed;
    if index < apps {
        Target::App(index)
    } else if index < tools {
        Target::Tool(index - apps)
    } else if index < stored {
        Target::Installed(index - tools)
    } else {
        Target::Package((index - stored).min(packages.saturating_sub(1)))
    }
}
