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

/// Why the scheduler is being asked to switch. `Preempt` is the timer
/// path: time slice ran out, give the CPU to whoever is next. `Yield`
/// is the voluntary path: caller is done for now, hand off to whoever
/// is next or stay if there is no one. A directed-target variant gets
/// added the day a real caller appears.
#[derive(Debug, Clone, Copy)]
pub enum SwitchIntent {
    Preempt,
    Yield,
}
