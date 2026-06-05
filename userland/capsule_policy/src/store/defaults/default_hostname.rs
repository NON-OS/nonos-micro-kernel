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

use super::constants::DEFAULT_HOSTNAME;
use crate::store::types::{StringField, STRING_CAP};

pub(super) const fn default_hostname() -> StringField {
    let mut bytes = [0u8; STRING_CAP];
    let src = DEFAULT_HOSTNAME;
    let mut i = 0;
    while i < src.len() {
        bytes[i] = src[i];
        i += 1;
    }
    StringField { bytes, len: src.len() }
}
