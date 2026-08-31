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

//! Reusable, geometry-parameterised chrome widgets. Every clickable widget
//! pairs its painter with a hit function so the two can never disagree.

mod docrow;
mod docrow_hit;
mod dropdown;
mod dropdown_hit;
mod navlist;
mod navlist_hit;
mod searchbox;
mod toggle;
mod toggle_hit;
mod truncate;

pub(in crate::editor) use docrow::{paint_docrow, DocRowStyle};
pub(in crate::editor) use docrow_hit::docrow_hit;
pub(in crate::editor) use dropdown::{dropdown_w, paint_dropdown, DropdownStyle};
pub(in crate::editor) use dropdown_hit::dropdown_hit;
pub(in crate::editor) use navlist::{nav_row_h, paint_navlist, NavStyle};
pub(in crate::editor) use navlist_hit::navlist_hit;
pub(in crate::editor) use searchbox::{paint_searchbox, searchbox_hit, SearchStyle};
pub(in crate::editor) use toggle::paint_toggle;
pub(in crate::editor) use toggle_hit::toggle_hit;
pub(in crate::editor) use truncate::truncate_to_width;
