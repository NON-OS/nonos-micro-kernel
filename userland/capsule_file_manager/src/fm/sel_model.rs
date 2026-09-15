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

use super::icon_path::Icon;

/// One action the selection band offers.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelAction {
    Share,
    Move,
    Duplicate,
    Compress,
    Tag,
    Delete,
    Clear,
}

/// The band's one table: the glyph, the label both the painter and the hit-test
/// measure from, and whether this crate has a handler behind the action.
///
/// `Share` and `Compress` are `false`: the vfs client exposes no archive op and
/// the capsule holds no identity or sharing channel, so neither can be wired
/// without inventing a backend. They draw dimmed and say so when clicked, which
/// is this crate's rule for an unwired control -- never a silent no-op.
pub const ACTIONS: [(SelAction, Icon, &str, bool); 7] = [
    (SelAction::Share, Icon::People, "Share", false),
    (SelAction::Move, Icon::Forward, "Move", true),
    (SelAction::Duplicate, Icon::Doc, "Duplicate", true),
    (SelAction::Compress, Icon::Archive, "Compress", false),
    (SelAction::Tag, Icon::Tag, "Tag", true),
    (SelAction::Delete, Icon::Trash, "Delete", true),
    (SelAction::Clear, Icon::Back, "Clear", true),
];
