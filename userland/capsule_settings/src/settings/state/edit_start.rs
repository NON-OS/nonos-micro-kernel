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

use super::cache::FieldValue;
use super::cached_value::cached_value;
use super::current_field::current_field;
use super::edit_buffer::EditBuffer;
use super::state::State;

pub fn edit_start(state: &mut State) {
    let field = match current_field(state) {
        Some(f) => f,
        None => return,
    };
    let mut edit = EditBuffer::empty();
    if let FieldValue::Str { bytes, len } = cached_value(state, field) {
        edit.bytes[..len].copy_from_slice(&bytes[..len]);
        edit.len = len;
    }
    state.edit = edit;
    state.editing = true;
}
