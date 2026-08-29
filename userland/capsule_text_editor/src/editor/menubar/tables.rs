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

//! Row tables. The row index is the action selector, so order is load-bearing.

use super::items::MenuCmd::{CloseTab, Ctrl, Todo, ToggleSidebar};
use super::items::MenuRow;

pub(super) const FILE: [MenuRow; 6] = [
    ("New Tab", Todo),
    ("Open...", Ctrl(0x4F, false)),
    ("Save", Ctrl(0x53, false)),
    ("Save As...", Ctrl(0x53, true)),
    ("Export...", Ctrl(0x45, false)),
    ("Close Tab", CloseTab),
];
pub(super) const EDIT: [MenuRow; 9] = [
    ("Undo", Ctrl(0x5A, false)),
    ("Redo", Ctrl(0x59, false)),
    ("Cut", Ctrl(0x58, false)),
    ("Copy", Ctrl(0x43, false)),
    ("Paste", Ctrl(0x56, false)),
    ("Select All", Ctrl(0x41, false)),
    ("Find...", Ctrl(0x46, false)),
    ("Replace...", Ctrl(0x48, false)),
    ("Replace All", Ctrl(0x48, true)),
];
pub(super) const VIEW: [MenuRow; 5] = [
    ("Zoom In", Ctrl(0x3D, false)),
    ("Zoom Out", Ctrl(0x2D, false)),
    ("Reset Zoom", Ctrl(0x30, false)),
    ("Toggle Theme", Ctrl(0x42, false)),
    ("Toggle Sidebar", ToggleSidebar),
];
pub(super) const INSERT: [MenuRow; 4] =
    [("Image", Todo), ("Link", Todo), ("Page Break", Todo), ("Special Character", Todo)];
pub(super) const FORMAT: [MenuRow; 4] =
    [("Bold", Todo), ("Italic", Todo), ("Underline", Todo), ("Toggle Comment", Ctrl(0x2F, false))];
pub(super) const TOOLS: [MenuRow; 3] = [
    ("Duplicate Line", Ctrl(0x44, false)),
    ("Delete Line", Ctrl(0x4B, true)),
    ("Word Count", Todo),
];
pub(super) const TABLE: [MenuRow; 4] =
    [("Insert Table", Todo), ("Insert Row", Todo), ("Insert Column", Todo), ("Delete Table", Todo)];
pub(super) const HELP: [MenuRow; 2] = [("Keyboard Shortcuts", Todo), ("About NONOS Docs", Todo)];
