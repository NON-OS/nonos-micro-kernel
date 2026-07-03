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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::tree::{BoxKind, BoxNode};

// A childless box (text or image) that inherits its parent's text style and
// carries the enclosing anchor. The parent background rides along so inline
// highlights survive the wrap into words.
pub(super) fn leaf(
    kind: BoxKind,
    parent: &Computed,
    link: &Option<String>,
    dom_id: usize,
) -> BoxNode {
    let mut style = Computed::inherit_from(parent);
    style.bg = parent.bg;
    BoxNode { kind, style, href: link.clone(), dom_id, children: Vec::new() }
}
