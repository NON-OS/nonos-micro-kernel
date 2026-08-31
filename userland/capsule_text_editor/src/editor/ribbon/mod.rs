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

//! The formatting ribbon under the menu bar: three dropdown pills, the run
//! style toggles, and the paragraph icons, plus the model edits behind them.

mod apply;
mod cell;
mod cells;
mod drop;
mod heading;
mod hit;
mod icons;
mod items;
mod metrics;
mod paint;
mod panel;
mod press;
mod snapshot;
mod toggle;

pub(in crate::editor) use drop::paint_ribbon_drop;
pub(in crate::editor) use metrics::RibbonCell;
pub(in crate::editor) use paint::paint_ribbon;
pub(in crate::editor) use snapshot::ribbon_state;
