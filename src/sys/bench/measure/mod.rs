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

//! Timing something small enough that milliseconds cannot see it.
//!
//! The boot markers next door report `uptime_ms`, which is the right unit for
//! phases and useless for anything a kernel does thousands of times a second.
//! An IPC round trip, a capability check, a signature verification: all of
//! those are microseconds or less, so measuring them needs the cycle counter
//! and needs the result reported as a spread rather than a single figure.

mod report;
mod run;
mod sample;

pub use report::report;
pub use run::measure;
pub use sample::Sample;
