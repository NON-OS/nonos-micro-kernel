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
use super::status::Status;

pub const FIELD_SLOTS: usize = 37;

pub struct State {
    pub policy_port: u32,
    pub policy_ready: bool,
    pub category: Category,
    pub cursor: [usize; 3],
    pub scroll_top: [usize; 3],
    pub values: [FieldValue; FIELD_SLOTS],
    pub editing: bool,
    pub edit: EditBuffer,
    pub status: Status,
}
