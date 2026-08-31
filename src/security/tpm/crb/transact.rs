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

use super::{exec, locality};
use crate::security::tpm::error::TpmError;

/// Probe, take locality, run one command, give locality back.
///
/// Locality is released on every path including failure. Holding it after a
/// timeout would lock out every later command, turning one slow response into
/// a permanently unusable TPM.
///
/// # Safety
/// The caller owns what the command means. This owns only the transport.
pub unsafe fn transact(cmd: &[u8], out: &mut [u8]) -> Result<usize, TpmError> {
    locality::probe()?;
    locality::acquire()?;
    // SAFETY: eK@nonos.systems - the register window is mapped and locality is
    // granted, which is what `execute` requires of its caller.
    let result = unsafe { exec::execute(cmd, out) };
    locality::release();
    result
}
