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

extern crate alloc;

use alloc::string::String;

use super::state::{PromptKind, State};
use super::undo::Op;

/// Record the inverse of a prompt operation that has already succeeded.
/// `prior` is the path the cursor was on before the operation ran, which is the
/// only thing a rename's inverse cannot be derived without.
///
/// A delete has no inverse: the vfs keeps neither a trash namespace nor a
/// content journal, so instead of pushing an op that would fail on replay the
/// whole stack is dropped, since the ops below it may name paths that are gone.
pub fn record(state: &mut State, kind: PromptKind, target: &str, name: &str, prior: Option<String>) {
    match kind {
        PromptKind::NewFile => state.undo.push(Op::Unlink { path: String::from(target) }),
        PromptKind::MkDir => state.undo.push(Op::Rmdir { path: String::from(target) }),
        PromptKind::Rename => {
            if let Some(old) = prior {
                let to = String::from(old.trim_end_matches('/'));
                state.undo.push(Op::Rename { from: String::from(target), to });
            }
        }
        PromptKind::Delete => {
            if name == "y" {
                state.undo.clear();
            }
        }
        PromptKind::Tag => {}
    }
}
