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

//! The document menu bar across the top of the shell: the title strip, the
//! dropdown panel for the open title, and the row tables behind both.

mod drop;
mod hit;
mod items;
mod metrics;
mod paint;
mod tables;

pub(in crate::editor) use drop::paint_dropdown;
pub(in crate::editor) use hit::{menubar_hit, MenuHit};
pub(in crate::editor) use items::{rows, MenuCmd};
pub(in crate::editor) use metrics::TitleSpan;
pub(in crate::editor) use paint::paint_menubar;
