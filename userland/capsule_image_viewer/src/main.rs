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

#[cfg(feature = "nonos-image-viewer-smoketest")]
mod selftest;
mod viewer;

use nonos_app_skeleton::run;

#[cfg(feature = "nonos-image-viewer-smoketest")]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    selftest::run()
}

#[cfg(not(feature = "nonos-image-viewer-smoketest"))]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    const VIEWER_HEAP: usize = 192 * 1024 * 1024;
    let _ = nonos_libc::heap_init_sized(VIEWER_HEAP);
    run(viewer::ViewerApp::new)
}
