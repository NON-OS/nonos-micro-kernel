// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::act::act;
use super::row::Row;
use crate::fwui::theme;
use crate::menu::MenuAction;
use alloc::vec;
use alloc::vec::Vec;

pub fn tool() -> Vec<Row> {
    vec![
        act(
            b"UEFI SHELL",
            ">",
            theme::TEXT,
            b"Exit to the UEFI shell environment.",
            MenuAction::UefiShell,
        ),
        act(
            b"MEMORY TEST",
            ">",
            theme::TEXT,
            b"Run a destructive RAM integrity test.",
            MenuAction::MemoryTest,
        ),
        act(
            b"HW DIAGNOSTICS",
            ">",
            theme::TEXT,
            b"Boot the hardware diagnostics environment.",
            MenuAction::Diagnostics,
        ),
        act(b"SHUTDOWN", ">", theme::WARN, b"Power off the platform.", MenuAction::Shutdown),
    ]
}
