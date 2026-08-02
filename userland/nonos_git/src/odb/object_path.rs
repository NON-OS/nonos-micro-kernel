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

//! Where an object's bytes live.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::oid::ObjectId;

/// `<git_dir>/objects/xx/yyy...`, the split id.
pub(super) fn object_path(git_dir: &str, id: &ObjectId) -> String {
    let (dir, file) = id.loose_path();
    format!("{git_dir}/objects/{dir}/{file}")
}
