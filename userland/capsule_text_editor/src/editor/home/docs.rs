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

//! The document list behind the Home pane. "Home" is every file the VFS
//! listing returned; "Recent" is what this session has actually opened. The
//! remaining nav rows have no store behind them and list nothing.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::super::app::Editor;

pub(super) struct DocRef {
    pub path: String,
    pub name: String,
}

pub(super) fn section_title(nav: usize) -> &'static str {
    match nav {
        1 => "Recent Documents",
        _ => "All Documents",
    }
}

pub(super) fn empty_line(nav: usize) -> &'static str {
    match nav {
        1 => "No documents opened yet this session.",
        _ => "No documents in the store.",
    }
}

pub(super) fn doc_list(ed: &Editor, nav: usize) -> Vec<DocRef> {
    let mut out = Vec::new();
    match nav {
        0 => {
            for node in ed.tree.nodes.iter().filter(|n| !n.is_dir) {
                out.push(DocRef { path: node.path.clone(), name: node.name.clone() });
            }
        }
        1 => {
            for path in ed.mru.iter() {
                out.push(DocRef { path: path.clone(), name: leaf(path).to_string() });
            }
        }
        _ => {}
    }
    out
}

fn leaf(path: &str) -> &str {
    let cut = path.rsplit('/').next().unwrap_or(path);
    match cut.is_empty() {
        true => path,
        false => cut,
    }
}
