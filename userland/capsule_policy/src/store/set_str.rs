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

use super::state::STORE;
use super::str_validate::validate;
use super::types::{StringField, STRING_CAP};

pub fn set(field: Field, bytes: &[u8]) -> bool {
    if bytes.len() > STRING_CAP {
        return false;
    }
    if !validate(bytes) {
        return false;
    }
    let mut sf = StringField { bytes: [0u8; STRING_CAP], len: bytes.len() };
    sf.bytes[..bytes.len()].copy_from_slice(bytes);
    let mut s = STORE.lock();
    match field {
        Field::Hostname => s.hostname = sf,
        Field::DomainName => s.domainname = sf,
        _ => return false,
    }
    true
}
