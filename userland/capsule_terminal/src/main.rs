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

#![no_std]
#![no_main]

extern crate alloc;

mod command;
mod event;
mod git;
mod jobs;
mod layout;
mod mixnet;
mod paint;
mod palette;
mod rail;
mod term;

#[cfg(not(feature = "nonos-autorun-selftest"))]
use nonos_app_skeleton::run;

/// # Safety
///
/// This is the capsule entry point. The loader calls it exactly once on a
/// freshly initialized stack with no live Rust state, and it never returns.
/// It must not be called from Rust code.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    #[cfg(feature = "nonos-autorun-selftest")]
    {
        term::terminal::selftest::main()
    }
    #[cfg(not(feature = "nonos-autorun-selftest"))]
    {
        run(term::Terminal::new)
    }
}
