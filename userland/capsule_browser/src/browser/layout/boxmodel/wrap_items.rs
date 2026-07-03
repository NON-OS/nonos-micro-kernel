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

use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::tree::BoxNode;
use super::wrap_runs::wrap_runs;

// Flex containers treat every child as an item: inline runs get an anonymous
// block each so the flex axis only ever sees block-level boxes.
pub(super) fn wrap_items(parent: &Computed, children: Vec<BoxNode>) -> Vec<BoxNode> {
    if children.iter().all(|c| c.kind.block_level()) {
        return children;
    }
    wrap_runs(parent, children)
}
