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

mod browser;
mod qjs_bridge;
mod qjs_dom;

use nonos_app_skeleton::run;

// The browser holds a page DOM, a box tree, decoded rasters and transient
// fetch buffers at once, so it claims a larger heap than the 16 MiB shared
// default before the skeleton initialises. A failure here is non-fatal: the
// skeleton's own init then falls back to the default size.
const BROWSER_HEAP: usize = 48 * 1024 * 1024;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let _ = nonos_libc::heap_init_sized(BROWSER_HEAP);
    run(browser::Browser::new)
}
