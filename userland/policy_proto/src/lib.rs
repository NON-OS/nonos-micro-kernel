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

#![no_std]

pub mod category;
pub mod cursor_size_labels;
pub mod enum_label;
pub mod enum_table;
pub mod errno;
pub mod field;
pub mod field_decode;
pub mod field_kind;
pub mod field_label;
pub mod field_max;
pub mod font_size_labels;
pub mod hdr;
pub mod keyboard_layout_labels;
pub mod kind;
pub mod language_labels;
pub mod limits;
pub mod ops;
pub mod proxy_mode_labels;
pub mod service;
pub mod theme_labels;
pub mod wallpaper_labels;

pub use category::Category;
pub use enum_label::enum_label;
pub use enum_table::enum_table;
pub use errno::{E_ACCES, E_BAD_LEN, E_INVAL, E_NOT_FOUND, E_OK, E_WRONG_KIND};
pub use field::Field;
pub use field_decode::decode as decode_field;
pub use field_kind::kind_of;
pub use field_label::label_of;
pub use field_max::max_of;
pub use hdr::{Header, HDR_LEN};
pub use kind::{KIND_BOOL, KIND_I8, KIND_STR, KIND_U8};
pub use limits::{IPC_PAYLOAD_MAX, STR_MAX};
pub use ops::{OP_GET, OP_SET};
pub use service::{POLICY_REPLY_PORT, POLICY_SERVICE_NAME, POLICY_SERVICE_PORT};
