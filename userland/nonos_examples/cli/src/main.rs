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

mod hex;
mod render;

use crate::render::render_line;
use nonos_runtime::prelude::*;

const CAPS: u64 = cap::CAP_CORE_EXEC | cap::CAP_MEMORY | cap::CAP_DEBUG | cap::CAP_CRYPTO;

fn main() {
    let _ = log::log(b"nonos cli example\n");
    let boot_ms = time_millis();
    let mut seed = [0u8; 4];
    let _ = entropy(&mut seed);
    render_line(boot_ms, &seed);
}

nonos_main!(CAPS, main);
