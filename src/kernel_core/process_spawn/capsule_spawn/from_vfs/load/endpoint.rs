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

use super::super::error::LoadError;
use crate::security::capsule_manifest::{CapsuleManifest, EndpointKind};

pub(super) fn endpoint(m: &CapsuleManifest, kind: EndpointKind) -> Result<(&str, u32), LoadError> {
    m.endpoints
        .iter()
        .find(|e| e.kind == kind)
        .map(|e| (e.name_str(), e.port))
        .ok_or(LoadError::Manifest)
}
