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

use super::super::error::ManifestVerifyError;
use super::super::schema::{CapsuleManifest, EndpointKind};

pub struct DeclaredEndpoint<'a> {
    pub kind: EndpointKind,
    pub port: u32,
    pub name: &'a str,
}

// Spawn site declares the (kind, port, name) tuples it is about to
// register. Manifest must declare every one of them; manifest may
// declare more (Reply with no Service-side, etc.) but the spawn
// site must not register an endpoint the manifest does not list.
pub(super) fn check(
    manifest: &CapsuleManifest,
    declared: &[DeclaredEndpoint<'_>],
) -> Result<(), ManifestVerifyError> {
    for d in declared {
        let found = manifest
            .endpoints
            .iter()
            .any(|e| e.kind == d.kind && e.port == d.port && e.name_str() == d.name);
        if !found {
            return Err(ManifestVerifyError::EndpointDeclDrift);
        }
    }
    Ok(())
}
