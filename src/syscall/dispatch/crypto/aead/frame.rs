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

use crate::syscall::dispatch::errno;
use crate::syscall::SyscallResult;

const MAX_AAD: usize = 256;

pub(super) fn split(frame: &[u8]) -> Result<(&[u8], &[u8]), SyscallResult> {
    if frame.len() < 4 {
        return Err(errno(22));
    }
    let aad_len = u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
    if aad_len > MAX_AAD || 4 + aad_len > frame.len() {
        return Err(errno(22));
    }
    Ok((&frame[4..4 + aad_len], &frame[4 + aad_len..]))
}
