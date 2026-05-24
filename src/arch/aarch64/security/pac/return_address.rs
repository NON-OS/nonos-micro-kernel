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

use core::arch::asm;

pub fn sign_return_address(lr: u64, sp: u64) -> u64 {
    let signed: u64;
    unsafe {
        asm!("paciasp", in("lr") lr, in("sp") sp, lateout("lr") signed);
    }
    signed
}

pub fn authenticate_return_address(lr: u64, sp: u64) -> u64 {
    let authenticated: u64;
    unsafe {
        asm!("autiasp", in("lr") lr, in("sp") sp, lateout("lr") authenticated);
    }
    authenticated
}
