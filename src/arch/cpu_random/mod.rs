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

//! The CPU's on-die random generator, read through the arch boundary.
//!
//! Every architecture NØNOS targets that has one exposes the same two taps,
//! so the boundary carries both rather than flattening them into one:
//!
//! * a conditioned DRBG output, cheap and always the first choice
//!   (`RDRAND` on x86_64, `RNDR` on aarch64);
//! * a reseeded draw straight off the entropy conditioner, slower and
//!   sometimes unavailable under load, but suitable for seeding a software
//!   DRBG (`RDSEED` on x86_64, `RNDRRS` on aarch64).
//!
//! Both reads are best effort. A `None` means the CPU declined this attempt or
//! has no generator at all; it never means "here is a value we made up". The
//! caller decides whether to retry, fall back to another entropy source, or
//! refuse to hand out a key.
//!
//! riscv64 reports no generator. Its equivalent is the Zkr `seed` CSR, and
//! probing that CSR is itself a machine-mode read that traps at S-mode unless
//! the SBI implementation forwards it. Reporting "absent" until the platform
//! layer can answer honestly is correct: callers fall through to their other
//! entropy sources rather than trusting a number nothing produced.

mod available;
mod read;

pub(crate) use available::{entropy_available, random_available};
pub(crate) use read::{entropy_u64, random_u64};
