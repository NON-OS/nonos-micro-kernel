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

use super::super::super::util::fatal_reset;
use crate::display::{log_error as panel_error, show_error_screen};
use uefi::prelude::*;

pub fn binding_failure(st: &mut SystemTable<Boot>, gop: bool, msg: &'static str) -> ! {
    if gop {
        panel_error(b"ZK BINDING FAILED");
        show_error_screen(msg.as_bytes());
    }
    fatal_reset(st, msg);
}
