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

//! The rest of the CPL=3-reachable faults.
//!
//! These kept their plain `extern "x86-interrupt"` wrappers while the others
//! were given trampolines, so a capsule taking one of them ran the handler on
//! the user GS base and any `gs:`-relative read in that handler, or anywhere
//! it called, read from the user base instead. A capsule reaches all five
//! without privilege: #NM by touching the FPU with CR0.TS set, #MF by an
//! unmasked x87 exception, #TS and #NP by loading a bad segment selector, and
//! #VE where EPT violation virtualisation is active.

use super::tramp_err::exc_tramp_err;
use super::tramp_noerr::exc_tramp_noerr;

exc_tramp_noerr!(nm_trampoline, nm_trap, crate::interrupts::handlers::device_not_available);
exc_tramp_noerr!(mf_trampoline, mf_trap, crate::interrupts::handlers::x87_floating_point);
exc_tramp_noerr!(ve_trampoline, ve_trap, crate::interrupts::handlers::virtualization);
exc_tramp_err!(ts_trampoline, ts_trap, crate::interrupts::handlers::invalid_tss);
exc_tramp_err!(np_trampoline, np_trap, crate::interrupts::handlers::segment_not_present);
