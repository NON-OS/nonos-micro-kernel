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

use nonos_policy_proto::Category;

use super::cache::FieldValue;
use super::edit_buffer::EditBuffer;
use super::state::{State, FIELD_SLOTS};
use super::status::Status;

pub fn new() -> State {
    State {
        policy_port: 0,
        policy_ready: false,
        category: Category::User,
        cursor: [0, 0, 0],
        scroll_top: [0, 0, 0],
        values: [FieldValue::Unknown; FIELD_SLOTS],
        editing: false,
        edit: EditBuffer::empty(),
        status: Status::idle(),
    }
}
