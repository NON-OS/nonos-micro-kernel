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

mod field_bool_value;
mod field_i8_value;
mod field_str_value;
mod field_u8_value;
pub mod fmt_dec;
pub mod fmt_signed;
pub mod layout;
pub mod paint;
pub mod paint_field_row;
pub mod paint_field_value;
pub mod paint_header;
pub mod paint_status;
pub mod paint_tabs;
pub mod paint_value_bool;
pub mod paint_value_enum;
pub mod paint_value_i8;
pub mod paint_value_str;
pub mod paint_value_u8;
pub mod scroll_indicator;
pub mod visible_rows;

pub use paint::paint;
