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

use alloc::sync::Arc;

use crate::capabilities::token::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

use super::super::args::SyscallArgs;
use super::check_asid::check_asid_binding;
use super::check_epoch::check_revocation_epoch;
use super::check_session::check_session_binding;
use super::check_syscall::check_syscall_allowed;
use super::check_token::check_token;
use super::context::ResolveContext;
use super::error::ResolverError;

pub(in crate::syscall::contract) fn resolve(
    token: &Arc<CapabilityToken>,
    number: SyscallNumber,
    _args: &SyscallArgs,
    ctx: &ResolveContext,
) -> Result<(), ResolverError> {
    check_token(token)?;
    check_session_binding(token, ctx)?;
    check_asid_binding(token, ctx)?;
    check_revocation_epoch(token, ctx)?;
    check_syscall_allowed(token, number)?;
    Ok(())
}
