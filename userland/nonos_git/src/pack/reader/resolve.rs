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
//! Applying a delta against an object already read.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;

use super::super::delta::apply;
use super::super::error::PackError;
use super::object::PackObject;

/// A delta takes its base's type: only the content differs.
///
/// No recursion is needed. A pack lists a base before the deltas naming it,
/// and this reads forward, so by the time a delta is reached its base is
/// already fully resolved rather than still a delta itself.
pub(super) fn resolve(
    seen: &[PackObject],
    is_base: impl Fn(&PackObject) -> bool,
    delta: &[u8],
) -> Result<(ObjectKind, Vec<u8>), PackError> {
    let base = seen.iter().find(|o| is_base(o)).ok_or(PackError::MissingBase)?;
    Ok((base.kind, apply(&base.data, delta)?))
}
