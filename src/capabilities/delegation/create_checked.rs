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

use crate::capabilities::token::{is_token_valid, CapabilityToken};
use crate::capabilities::types::Capability;

use super::error::DelegationError;
use super::lifetime::delegation_expiry;
use super::sign::sign_delegation;
use super::types::Delegation;

pub fn create_delegation(
    parent: &CapabilityToken,
    delegatee: u64,
    caps: &[Capability],
    ttl_ms: Option<u64>,
) -> Result<Delegation, DelegationError> {
    if caps.is_empty() {
        return Err(DelegationError::NoCapabilities);
    }
    if !is_token_valid(parent) {
        if !parent.not_expired() {
            return Err(DelegationError::ParentExpired);
        }
        return Err(DelegationError::InvalidParentToken);
    }
    for cap in caps {
        if !parent.grants(*cap) {
            return Err(DelegationError::CapabilityNotHeld);
        }
    }
    let now = crate::time::timestamp_millis();
    let requested = ttl_ms.map(|t| now.saturating_add(t));
    let expiry = delegation_expiry(requested, parent.expires_at_ms);
    let mut delegation = Delegation {
        delegator: parent.owner_module,
        delegatee,
        capabilities: caps.to_vec(),
        expires_at_ms: expiry,
        parent_nonce: parent.nonce,
        signature: [0u8; 64],
    };
    sign_delegation(&mut delegation)?;
    Ok(delegation)
}
