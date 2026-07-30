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

/// Which way the machine leaves this boot.
///
/// Both ends of the machine, not just the quiet one: a warm reset keeps DRAM
/// powered and its rows readable, so it needs the same treatment on the way
/// out as a power-off does.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum PowerOff {
    /// Cut power. The firmware never gets control back.
    Shutdown,
    /// Reset the machine and boot again.
    Reboot,
}
