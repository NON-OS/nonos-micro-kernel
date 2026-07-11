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
use super::preview::Preview;

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
}
