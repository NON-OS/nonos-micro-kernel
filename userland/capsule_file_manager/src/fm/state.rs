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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use super::clipboard::Clip;
use super::entries::Entry;
use super::favorites::Favorites;
use super::prefs::Prefs;
use super::preview::Preview;
use super::screen::Screen;
use super::undo::UndoStack;

#[derive(Clone, Copy)]
pub enum Mode {
    Browse,
    Filter,
    Help,
    Prompt(PromptKind),
    Preview,
}

#[derive(Clone, Copy)]
pub enum PromptKind {
    NewFile,
    MkDir,
    Rename,
    Delete,
    // Sidecar metadata rather than a filesystem op: this one names a tag to
    // toggle on the cursor path, not a path to create.
    Tag,
}

#[derive(Clone, Copy)]
pub enum SortMode {
    Name,
    Size,
    Date,
    Type,
}

/// How the current directory is presented: a grid of large icons (the default,
/// matching a modern desktop file manager) or a compact detail list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewKind {
    Grid,
    List,
}

pub struct State {
    pub owner_pid: u32,
    pub prefix: String,
    pub all: Vec<Entry>,
    pub entries: Vec<Entry>,
    pub cursor: usize,
    // Index of the first entry drawn, so long directories scroll instead of
    // clipping the rows that fall past the window.
    pub scroll: usize,
    pub preview: Option<Preview>,
    pub status: &'static [u8],
    pub mode: Mode,
    pub input: String,
    pub filter: String,
    pub sort_mode: SortMode,
    pub selected: Vec<String>,
    pub clipboard: Vec<Clip>,
    // Live layout geometry, recomputed each paint from the window size so scroll
    // clamping and click-to-row stay consistent with what is drawn.
    pub view_rows: usize,
    pub row_top: u32,
    pub row_h: u32,
    // Grid presentation and its live column count, recomputed each paint so a
    // click maps to the same cell that was drawn.
    pub view: ViewKind,
    pub grid_cols: u32,
    // Width of the surface the last frame was painted into. The header's control
    // strip is right-aligned, so its hit-test has no PaintBuffer to ask and must
    // read the same width the painter used or every control drifts on a resize.
    pub win_w: u32,
    // Height of that same surface, for the stacked screens whose hit-test has to
    // clip to the same bottom the painter did.
    pub win_h: u32,
    // The vfs walk behind the info panel's directory rows, keyed by the path it
    // describes. Paint must never block on IPC, so the walk runs once per cursor
    // path and a refresh drops it; `None` inside the pair is a cached failure.
    pub dir_info: Option<(String, Option<(u32, u32, u64, bool)>)>,
    // Store occupancy as the vfs reports it: (files, bytes_used, max_files).
    // Read once per refresh, like `dir_info`, because the sidebar's drive card
    // draws every frame and paint must never block on IPC.
    pub usage: Option<(u32, u64, u32)>,
    pub tags: super::tags::TagMap,
    pub tag_filter: String,
    // Which top-level surface is drawn, and the persisted sidecar state the
    // surfaces read: pinned paths, saved preferences, and the in-memory
    // inverse stack behind the Undo control.
    pub screen: Screen,
    pub favorites: Favorites,
    pub prefs: Prefs,
    pub undo: UndoStack,
    // Search surface: the live query and its hits as (kind, line, path), where
    // line is the 1-based match line for a content hit and 0 otherwise.
    pub query: String,
    pub hits: Vec<(u32, u32, String)>,
    // The active filetype chip on each filtering surface; `None` is the All
    // chip. Only kinds the data actually holds are ever offered, so a filter can
    // never select an empty list.
    pub hit_filter: Option<super::filetype::Kind>,
    pub recents_filter: Option<super::filetype::Kind>,
    // Journal entries as (millis, path), newest first, grouped by the Recents
    // and Home surfaces.
    pub recents: Vec<(u64, String)>,
    // One cached `dirstat` walk per store prefix the Home cards state a figure
    // for, keyed by that prefix. Taken while Home is showing and dropped by a
    // refresh, because paint must never block on IPC; `None` inside the pair is
    // a cached failure and is reported as one rather than shown as zero.
    pub place_stats: Vec<(String, Option<(u32, u32, u64, bool)>)>,
}
