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

// A block container must hold only blocks: when block and inline children
// mix, consecutive inline runs get an anonymous block around them.
pub(super) fn wrap_mixed(parent: &Computed, children: Vec<BoxNode>) -> Vec<BoxNode> {
    let has_block = children.iter().any(|c| c.kind.block_level());
    if !has_block {
        return children;
    }
    wrap_runs(parent, children)
}
