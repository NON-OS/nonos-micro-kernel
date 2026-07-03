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

use super::tree::{BoxKind, BoxNode};

// Wrap a pending run of inline children in one anonymous block and append it.
pub(super) fn flush_run(out: &mut Vec<BoxNode>, run: &mut Vec<BoxNode>, parent: &Computed) {
    if run.is_empty() {
        return;
    }
    out.push(BoxNode {
        kind: BoxKind::Block,
        style: Computed::inherit_from(parent),
        href: None,
        dom_id: 0,
        children: core::mem::take(run),
    });
}
