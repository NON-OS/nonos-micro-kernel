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

use super::draw_body::draw_body;
use super::draw_full::draw_full;
use super::read_time::read_time;
use crate::display::gop::{get_dimensions, is_initialized};
use crate::fwui::chrome::clock;
use crate::fwui::data::gather;
use crate::fwui::input::{poll, Nav};
use crate::fwui::layout::compute;
use crate::fwui::metrics::set_from_screen;
use crate::fwui::section::rows_for;
use crate::fwui::settings::{apply, load, mode_from, save};
use crate::fwui::state::{Section, UiState};
use crate::hardware::HardwareInfo;
use crate::menu::{MenuAction, SecurityMode};
use crate::security::SecurityContext;
use uefi::prelude::*;

pub fn run(st: &mut SystemTable<Boot>, sec: &SecurityContext, hw: &HardwareInfo) -> MenuAction {
    if !is_initialized() {
        return MenuAction::Timeout;
    }
    set_from_screen(get_dimensions().1);
    let sys = gather(st, sec, hw);
    let mut settings = load(st);
    let lay = compute();
    let mut ui = UiState::new(settings.timeout_s as u64 * 1000);
    let (mut full, mut body, mut last_sec) = (true, true, 60u8);
    loop {
        let now = read_time(st);
        if full {
            draw_full(&lay, &sys, &now, ui.section());
            full = false;
            body = true;
        }
        let rows = rows_for(ui.section(), &sys, &settings, &now);
        ui.clamp(rows.len());
        if body {
            draw_body(&lay, &sys, ui.section(), &rows, ui.row());
            body = false;
        }
        if now.second() != last_sec {
            last_sec = now.second();
            clock(&lay, &sys, &now);
            if ui.section() == Section::Main {
                body = true;
            }
        }
        match poll(st.boot_services()) {
            Nav::Up => {
                ui.move_up(rows.len());
                body = true;
            }
            Nav::Down => {
                ui.move_down(rows.len());
                body = true;
            }
            Nav::Left => {
                ui.prev_section();
                full = true;
            }
            Nav::Right => {
                ui.next_section();
                full = true;
            }
            Nav::Enter => {
                let cur = rows.get(ui.row());
                if let Some(e) = cur.and_then(|r| r.edit) {
                    apply(&mut settings, e);
                    save(st, &settings);
                    ui.touch();
                    body = true;
                } else if let Some(a) = cur.and_then(|r| r.action) {
                    return a;
                }
            }
            Nav::Back => return MenuAction::Continue,
            Nav::BootNow => return MenuAction::Boot(SecurityMode::Standard),
            Nav::None => {}
        }
        st.boot_services().stall(50_000);
        ui.elapsed_ms = ui.elapsed_ms.saturating_add(50);
        if ui.timed_out() {
            return MenuAction::Boot(mode_from(settings.default_mode));
        }
    }
}
