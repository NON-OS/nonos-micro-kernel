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

//! The file-explorer model. One `vfs::list_paths("/")` returns every path in
//! the store as a flat list (directories suffixed `/`), which becomes a
//! collapsible tree: `nodes` holds every entry, `expanded` records which
//! directories are open, and `visible` is the depth-first row order shown.

mod activate;
mod new;
mod parent;
mod reload;
mod visible;

pub(in crate::editor) use parent::parent_of;

use alloc::string::String;
use alloc::vec::Vec;

pub struct Node {
    pub path: String, // canonical, no trailing slash: "/src" or "/src/main.rs"
    pub name: String,
    pub is_dir: bool,
    pub depth: u16,
}

pub struct FileTree {
    pub nodes: Vec<Node>,
    pub expanded: Vec<String>,
    pub visible: Vec<usize>,
    pub scroll: u32,
    pub selected: usize, // index into `visible`; usize::MAX when nothing is selected
    pub loaded: bool,
    pub status: &'static str,
}
