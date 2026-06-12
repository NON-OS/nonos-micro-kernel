// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::row::Row;
use crate::fwui::settings::Edit;
use crate::fwui::theme;
use alloc::string::String;

pub fn set(label: &'static [u8], value: String, desc: &'static [u8], edit: Edit) -> Row {
    Row { label, value, vcolor: theme::ACCENT, desc, action: None, edit: Some(edit) }
}
