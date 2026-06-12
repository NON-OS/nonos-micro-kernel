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

use crate::capabilities::token::CapabilityToken;
use crate::crypto::util::constant_time::ct_eq_16;

use super::context::ResolveContext;
use super::error::ResolverError;

pub(super) fn check_session_binding(
    token: &CapabilityToken,
    ctx: &ResolveContext,
) -> Result<(), ResolverError> {
    let live = ctx.boot_session_nonce.ok_or(ResolverError::BootSessionNotLatched)?;
    if !ct_eq_16(&token.boot_session_nonce, &live) {
        return Err(ResolverError::BootSessionMismatch);
    }
    Ok(())
}
