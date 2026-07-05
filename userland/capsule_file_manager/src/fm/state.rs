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
    Prompt(PromptKind),
    Preview,
    Filter,
    Help,
}

#[derive(Clone, Copy)]
pub enum PromptKind {
    NewFile,
    MkDir,
    Rename,
    Delete,
    Goto,
}

// Order the listing is presented in. Cycled with the sort key.
#[derive(Clone, Copy, PartialEq)]
pub enum SortMode {
    Name,
    Size,
    Date,
    Type,
}

impl SortMode {
    pub fn next(self) -> Self {
        match self {
            SortMode::Name => SortMode::Size,
            SortMode::Size => SortMode::Date,
            SortMode::Date => SortMode::Type,
            SortMode::Type => SortMode::Name,
        }
    }

    pub fn label(self) -> &'static [u8] {
        match self {
            SortMode::Name => b"name",
            SortMode::Size => b"size",
            SortMode::Date => b"date",
            SortMode::Type => b"type",
        }
    }
}

pub struct State {
    pub owner_pid: u32,
    pub prefix: String,
    // Every immediate child of the current directory, as listed.
    pub all: Vec<Entry>,
    // The filtered and sorted slice of `all` currently shown; the cursor and
    // scroll index into this view.
    pub entries: Vec<Entry>,
    pub cursor: usize,
    // Index of the first entry drawn, so long directories scroll instead of
    // clipping the rows that fall past the window.
    pub scroll: usize,
    pub preview: Option<Preview>,
    // Full paths checked for a batch action; empty means act on the cursor.
    pub selected: Vec<String>,
    // Entries awaiting a paste; more than one after a multi-select copy or cut.
    pub clipboard: Vec<Clip>,
    pub sort_mode: SortMode,
    // Live substring filter applied to entry names; empty shows everything.
    pub filter: String,
    // Store occupancy (files, bytes_used, max_files) for the capacity line.
    pub usage: Option<(u32, u64, u32)>,
    // Whether the current sort is reversed.
    pub sort_rev: bool,
    pub status: &'static [u8],
    pub mode: Mode,
    pub input: String,
}

impl State {
    pub fn new() -> Self {
        State {
            owner_pid: 0,
            prefix: String::from("/"),
            all: Vec::new(),
            entries: Vec::new(),
            cursor: 0,
            scroll: 0,
            preview: None,
            selected: Vec::new(),
            clipboard: Vec::new(),
            sort_mode: SortMode::Name,
            filter: String::new(),
            usage: None,
            sort_rev: false,
            status: b"loading...",
            mode: Mode::Browse,
            input: String::new(),
        }
    }
}
