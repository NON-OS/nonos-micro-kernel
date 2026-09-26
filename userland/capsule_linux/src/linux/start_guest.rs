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

//! Proving a program, laying it out, and making it runnable.

use nonos_libc::mk_foreign_start;

use super::guest::Guest;
use super::guest::{STACK_SIZE, STACK_TOP};
use super::image;
use super::origin::Origin;
use super::start::say;

pub(super) fn start(
    guest: &mut Guest,
    path: &[u8],
    bytes: &[u8],
    origin: Origin,
) -> Result<(), &'static [u8]> {
    if let Err(why) = prove(path, bytes, origin) {
        say(b"[LINUX] refused: ");
        say(why.as_bytes());
        say(b"\n");
        return Err(&b"[LINUX] unproven program\n"[..]);
    }
    let (image, entry, interp_base) = image::program(guest, bytes).map_err(|e| e.why())?;
    guest.map(STACK_TOP - STACK_SIZE, STACK_SIZE, true, false);
    let argv = alloc::vec![path.to_vec()];
    let rsp = image::build(guest, STACK_TOP, &image, interp_base, &argv, &super::env::default())
        .ok_or(&b"[LINUX] stack refused\n"[..])?;
    match mk_foreign_start(guest.pid, entry, rsp) {
        n if n < 0 => Err(&b"[LINUX] start refused\n"[..]),
        _ => Ok(()),
    }
}

/// A program out of the store proves itself against the enrolled set.
fn prove(path: &[u8], bytes: &[u8], origin: Origin) -> Result<(), &'static str> {
    match origin {
        Origin::BuiltIn => Ok(()),
        Origin::Store => super::attest::verify(path, bytes).map(|_| ()),
    }
}
