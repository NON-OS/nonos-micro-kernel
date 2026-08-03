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
//! What every step of a pull needs to know.

use super::args::PullArgs;

/// The three values that were being threaded through every call: who is
/// pulling, where from, and under what options. They travel together and
/// never change during a pull, so they travel as one thing.
pub struct Ctx<'a> {
    pub pid: u32,
    pub ip: [u8; 4],
    pub args: &'a PullArgs,
}
