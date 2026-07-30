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

/// What one CPU is telling another to do.
///
/// The set is closed on purpose: every kind here has a handler installed at
/// boot on each arch, so adding one means adding that handler too rather than
/// silently sending an interrupt nobody answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipi {
    /// Drop the TLB entry the shootdown record names.
    TlbShootdown,
    /// Look at the run queue again.
    Reschedule,
    /// Run the queued cross-CPU call.
    CallFunction,
    /// Everyone check in at the rendezvous point.
    Barrier,
    /// The system is panicking; stop what you are doing.
    Panic,
    /// Halt and stay halted.
    Stop,
}
