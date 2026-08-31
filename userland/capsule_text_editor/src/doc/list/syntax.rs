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

//! The list's stored form: the marker is part of the line's own text, `- ` for
//! a bullet and `N. ` for a numbered item, so a list survives the rebuild
//! `reflow` performs on every keystroke and every caret offset stays real.

pub const BULLET: &[u8] = b"- ";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Bullet,
    Number,
}

pub fn bullet_len(line: &[u8]) -> usize {
    match line.starts_with(BULLET) {
        true => BULLET.len(),
        false => 0,
    }
}

pub fn number_len(line: &[u8]) -> usize {
    let digits = line.iter().take_while(|c| c.is_ascii_digit()).count();
    match digits > 0 && line.get(digits) == Some(&b'.') && line.get(digits + 1) == Some(&b' ') {
        true => digits + 2,
        false => 0,
    }
}

pub fn marker_len(line: &[u8]) -> usize {
    match bullet_len(line) {
        0 => number_len(line),
        n => n,
    }
}

pub fn kind_of(line: &[u8]) -> Option<ListKind> {
    if bullet_len(line) > 0 {
        return Some(ListKind::Bullet);
    }
    match number_len(line) {
        0 => None,
        _ => Some(ListKind::Number),
    }
}
