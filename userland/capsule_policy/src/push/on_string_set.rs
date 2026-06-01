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

use super::kernel_field::{DOMAIN_NAME, HOSTNAME, KIND_STR};
use super::raw;

pub fn on_string_set(field: Field, value: &[u8]) {
    let field_id = match field {
        Field::Hostname => HOSTNAME,
        Field::DomainName => DOMAIN_NAME,
        _ => return,
    };
    let _ = raw::submit(field_id, KIND_STR, value);
}
