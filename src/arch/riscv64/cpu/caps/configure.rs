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

use core::sync::atomic::Ordering;

use crate::arch::fdt::find::isa::IsaFlags;

use super::bits::{CAP_A, CAP_C, CAP_CONFIGURED, CAP_D, CAP_F, CAP_V};
use super::state::CAPS;

pub fn configure(flags: IsaFlags) {
    let mut bits = CAP_CONFIGURED;
    if flags.f {
        bits |= CAP_F;
    }
    if flags.d {
        bits |= CAP_D;
    }
    if flags.v {
        bits |= CAP_V;
    }
    if flags.a {
        bits |= CAP_A;
    }
    if flags.c {
        bits |= CAP_C;
    }
    CAPS.store(bits, Ordering::Release);
}
