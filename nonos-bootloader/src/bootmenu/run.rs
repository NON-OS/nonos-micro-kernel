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

use uefi::prelude::*;

use super::entries::ENTRIES;
use super::input::poll;
use super::nav::Nav;
use super::render::render;
use crate::display::gop::is_initialized;
use crate::hardware::HardwareInfo;
use crate::menu::MenuAction;
use crate::security::SecurityContext;

const TIMEOUT_S: u32 = 10;
const TICK_US: usize = 50_000;
const TICKS_PER_S: u32 = 20;

pub fn run(st: &mut SystemTable<Boot>, sec: &SecurityContext, _hw: &HardwareInfo) -> MenuAction {
    if !is_initialized() {
        return MenuAction::Timeout;
    }
    let mut sel = 0usize;
    let mut ticks = 0u32;
    let mut shown_prev = u32::MAX;
    let mut counting = true;
    loop {
        let remaining = if counting { TIMEOUT_S.saturating_sub(ticks / TICKS_PER_S) } else { 0 };
        if counting && remaining == 0 {
            return ENTRIES[0];
        }
        if remaining != shown_prev {
            render(sel, remaining, sec);
            shown_prev = remaining;
        }
        match poll(st.boot_services()) {
            Nav::Up => {
                sel = if sel == 0 { ENTRIES.len() - 1 } else { sel - 1 };
                counting = false;
                shown_prev = u32::MAX;
            }
            Nav::Down => {
                sel = (sel + 1) % ENTRIES.len();
                counting = false;
                shown_prev = u32::MAX;
            }
            Nav::Enter => return ENTRIES[sel],
            Nav::None => {}
        }
        st.boot_services().stall(TICK_US);
        if counting {
            ticks += 1;
        }
    }
}
