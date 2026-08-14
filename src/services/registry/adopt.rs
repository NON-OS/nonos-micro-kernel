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

use super::error::RegError;
use super::ENDPOINTS;

/// Give an unclaimed endpoint its owner and its entry requirement.
///
/// A capsule's reply inbox has to be registered before the capsule exists,
/// because the name is needed to build the process and the pid does not exist
/// until after. It is therefore created unowned, and stays that way for the
/// few instructions until this claims it.
///
/// Only an endpoint still at pid zero can be adopted, so this cannot be used
/// to take an inbox that already belongs to somebody. That restriction is the
/// whole of its safety: without it, adoption would be a way to redirect
/// another capsule's replies to yourself.
pub(crate) fn adopt_endpoint(name: &str, pid: u32, caps: u64) -> Result<(), RegError> {
    if pid == 0 {
        return Err(RegError::PermissionDenied);
    }
    let mut eps = ENDPOINTS.lock();
    let ep = eps
        .iter_mut()
        .find(|e| e.name == name)
        .ok_or(RegError::NotFound)?;
    if ep.pid != 0 {
        return Err(RegError::PermissionDenied);
    }
    ep.pid = pid;
    ep.caps_required = caps;
    Ok(())
}
