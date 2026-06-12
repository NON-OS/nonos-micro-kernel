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

use crate::fwui::settings::Edit;
use crate::menu::MenuAction;
use alloc::string::String;

pub struct Row {
    pub label: &'static [u8],
    pub value: String,
    pub vcolor: u32,
    pub desc: &'static [u8],
    pub action: Option<MenuAction>,
    pub edit: Option<Edit>,
}
