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

//! Building a marker and keeping an offset honest across a splice. Both are
//! pure so the toggle can plan an edit before it commits one.

use alloc::format;
use alloc::vec::Vec;

use crate::doc::list::syntax::{ListKind, BULLET};

pub fn marker(kind: ListKind, index: usize) -> Vec<u8> {
    match kind {
        ListKind::Bullet => BULLET.to_vec(),
        ListKind::Number => format!("{}. ", index).into_bytes(),
    }
}

pub fn shifted(caret: usize, at: usize, del: usize, ins: usize) -> usize {
    if caret <= at {
        return caret;
    }
    match caret < at + del {
        true => at + ins,
        false => caret - del + ins,
    }
}
