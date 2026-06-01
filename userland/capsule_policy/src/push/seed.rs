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

use nonos_policy_proto::Field;
use crate::store::{get_bool, get_i8, get_str, STRING_CAP};

use super::{on_bool_set, on_i8_set, on_string_set};

pub fn seed_kernel() {
    if let Some(v) = get_bool::get(Field::KernelPreempt) {
        on_bool_set(Field::KernelPreempt, v);
    }
    if let Some(v) = get_i8::get(Field::Timezone) {
        on_i8_set(Field::Timezone, v);
    }
    let mut buf = [0u8; STRING_CAP];
    if let Some(n) = get_str::get(Field::Hostname, &mut buf) {
        on_string_set(Field::Hostname, &buf[..n]);
    }
    let mut buf2 = [0u8; STRING_CAP];
    if let Some(n) = get_str::get(Field::DomainName, &mut buf2) {
        on_string_set(Field::DomainName, &buf2[..n]);
    }
}
