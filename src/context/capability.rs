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

use super::current::get_current_context;
use super::error::ContextError;
use super::types::ExecutionContext;

/// # Safety
/// Checks capability without fallback to full privileges. This is critical
/// for security - the function MUST return an error when no context exists.
/// Kernel context has all capabilities. Process context uses stored caps.
/// Never grants implicit privileges when context is None.
pub fn has_capability(cap: u64) -> Result<bool, ContextError> {
    match get_current_context() {
        ExecutionContext::None => Err(ContextError::NoActiveContext),
        ExecutionContext::Kernel(_) => Ok(true),
        ExecutionContext::Process(ctx) => Ok(ctx.has_capability(cap)),
    }
}

/// # Safety
/// Requires capability or returns error. This is a security gate - callers
/// must handle the error case. Never falls back to granting access when
/// capability check fails or context is missing.
pub fn require_capability(cap: u64) -> Result<(), ContextError> {
    match has_capability(cap)? {
        true => Ok(()),
        false => Err(ContextError::CapabilityDenied),
    }
}
