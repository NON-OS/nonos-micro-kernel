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

//! A trailer for something this machine is installing.

use crate::syscall::{call_raw, N_MK_APP_INSTALL, N_MK_LOCAL_SIGN, N_MK_LOCAL_VERIFY};

/// How many bytes a trailer for `elf` takes, or a negative errno.
pub fn mk_local_sign_len(elf: &[u8], caps: u64) -> i64 {
    call_raw(N_MK_LOCAL_SIGN, [elf.as_ptr() as u64, elf.len() as u64, caps, 0, 0, 0])
}

/// Mint a trailer proving this machine may run `elf` holding `caps`, into
/// `out`.
pub fn mk_local_sign(elf: &[u8], caps: u64, out: &mut [u8]) -> i64 {
    let args =
        [elf.as_ptr() as u64, elf.len() as u64, caps, out.as_mut_ptr() as u64, out.len() as u64, 0];
    call_raw(N_MK_LOCAL_SIGN, args)
}

/// True when `elf` is proved under a root this machine trusts for exactly
/// `caps`.
pub fn mk_local_verify(elf: &[u8], caps: u64, trailer: &[u8]) -> bool {
    let args = [
        elf.as_ptr() as u64,
        elf.len() as u64,
        caps,
        trailer.as_ptr() as u64,
        trailer.len() as u64,
        0,
    ];
    call_raw(N_MK_LOCAL_VERIFY, args) == 0
}

/// Ask for a distribution package to be installed.
pub fn mk_app_install(package: &[u8]) -> i64 {
    call_raw(N_MK_APP_INSTALL, [package.as_ptr() as u64, package.len() as u64, 0, 0, 0, 0])
}

