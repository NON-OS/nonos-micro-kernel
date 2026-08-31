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


//! The two reference sheets Help opens as documents: the shortcut table the
//! capsule really dispatches, and what the editor is and is not.

pub(super) const SHORTCUTS: &str = concat!(
    "# Keyboard Shortcuts\n\n",
    "File\n",
    "Ctrl-O open   Ctrl-S save   Ctrl-Shift-S save as   Ctrl-E export\n\n",
    "Edit\n",
    "Ctrl-Z undo   Ctrl-Y redo   Ctrl-X cut   Ctrl-C copy   Ctrl-V paste\n",
    "Ctrl-A select all   Ctrl-F find   Ctrl-H replace   Ctrl-Shift-H replace all\n\n",
    "Lines\n",
    "Ctrl-D duplicate line   Ctrl-Shift-K delete line   Ctrl-/ toggle comment\n\n",
    "View\n",
    "Ctrl-= zoom in   Ctrl-- zoom out   Ctrl-0 reset zoom   Ctrl-B cycle theme\n\n",
    "Format\n",
    "Ctrl-Shift-B bold   Ctrl-U underline\n",
);

pub(super) const ABOUT: &str = concat!(
    "# About NONOS Docs\n\n",
    "The NONOS text editor capsule: a signed userland document editor running\n",
    "at CPL=3 over the microkernel's capability boundary.\n\n",
    "The text buffer is the document. Headings are stored as a leading # so\n",
    "they survive every edit; bold, underline, strike, font and size are run\n",
    "styles on the laid-out model and are rebuilt from the text on the next\n",
    "keystroke.\n\n",
    "Controls drawn dimmed have no document model behind them yet.\n",
);
