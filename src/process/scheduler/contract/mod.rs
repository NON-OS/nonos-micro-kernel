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

//! Scheduler context-switch contract. Single entry, witness-gated.
//! `switch(intent)` checks the precondition (interrupts off), mints a
//! `SwitchLease`, and hands off to the per-arch backend. Today only
//! x86_64 has a backend; aarch64 / riscv64 add one when their entry
//! shims land.

mod backend;
mod intent;
mod lease;
mod outcome;
mod switch;

pub use intent::SwitchIntent;
pub use lease::SwitchLease;
pub use outcome::{SwitchError, SwitchOutcome};
pub use switch::switch;
