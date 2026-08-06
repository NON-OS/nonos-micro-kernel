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

//! Writing a signature line.

extern crate alloc;

use alloc::vec::Vec;

use crate::commit::offset;

use super::decimal::push;
use super::types::Signature;

impl Signature {
    pub(in crate::commit) fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.name.as_bytes());
        out.extend_from_slice(b" <");
        out.extend_from_slice(self.email.as_bytes());
        out.extend_from_slice(b"> ");
        push(out, self.when);
        out.push(b' ');
        offset::write(out, self.offset_minutes);
    }
}
