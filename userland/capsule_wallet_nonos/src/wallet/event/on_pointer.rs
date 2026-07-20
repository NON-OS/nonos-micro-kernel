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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::{State, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED};

pub fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let x = x as u32;
    let y = y as u32;
    if state.locked {
        state.locked = false;
        return EventOutcome::Repaint;
    }
    if header_icon(state, x, y) {
        return EventOutcome::Repaint;
    }
    if state.panel != 0 {
        return panel_click(state, x, y);
    }
    if nav(state, x, y) {
        return EventOutcome::Repaint;
    }
    match state.view {
        VIEW_HOME => super::on_pointer_view::home(state, x, y),
        VIEW_RECEIVE => super::on_pointer_view::receive(state, x, y),
        VIEW_SEND => super::on_pointer_view::send(state, x, y),
        VIEW_PROOF => super::on_pointer_view::proof(state, x, y),
        VIEW_NOX => super::on_pointer_view::nox(state, x, y),
        _ => EventOutcome::Idle,
    }
}

fn header_icon(state: &mut State, x: u32, y: u32) -> bool {
    use crate::wallet::paint::{
        CMD_BTN_X, HDR_H, HDR_Y, ICON_W, LOCK_BTN_X, MAIN_BTN_X, MAIN_W, MSG_BTN_X, THEME_BTN_X,
    };
    if hit(x, y, THEME_BTN_X, HDR_Y, ICON_W, HDR_H) {
        state.light_mode = !state.light_mode;
    } else if hit(x, y, CMD_BTN_X, HDR_Y, ICON_W, HDR_H) {
        state.panel = if state.panel == 1 { 0 } else { 1 };
    } else if hit(x, y, MSG_BTN_X, HDR_Y, ICON_W, HDR_H) {
        state.panel = if state.panel == 2 { 0 } else { 2 };
    } else if hit(x, y, LOCK_BTN_X, HDR_Y, ICON_W, HDR_H) {
        state.locked = true;
        state.panel = 0;
    } else if hit(x, y, MAIN_BTN_X, HDR_Y, MAIN_W, HDR_H) {
        state.panel = if state.panel == 3 { 0 } else { 3 };
    } else {
        return false;
    }
    true
}

fn panel_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    if state.panel == 1 {
        for i in 0..8u32 {
            if hit(x, y, 410, 240 + i * 36, 460, 36) {
                run_cmd(state, i as u8);
                state.panel = 0;
                return EventOutcome::Repaint;
            }
        }
    } else if state.panel == 3 {
        for i in 0..3u32 {
            if hit(x, y, 1002, 122 + i * 38, 244, 34) {
                state.account = i as u8;
                state.panel = 0;
                return EventOutcome::Repaint;
            }
        }
    }
    state.panel = 0;
    EventOutcome::Repaint
}

fn run_cmd(state: &mut State, i: u8) {
    let views = [VIEW_HOME, VIEW_RECEIVE, VIEW_SEND, VIEW_PROOF, VIEW_SHIELDED, VIEW_NOX];
    if (i as usize) < views.len() {
        state.view = views[i as usize];
    } else if i == 6 {
        state.light_mode = !state.light_mode;
    } else if i == 7 {
        state.locked = true;
    }
}

fn nav(state: &mut State, x: u32, y: u32) -> bool {
    use crate::wallet::paint::{NAV_H, NAV_STEP, NAV_W, NAV_X, NAV_Y0};
    if x < NAV_X || x >= NAV_X + NAV_W || y < NAV_Y0 {
        return false;
    }
    let rel = y - NAV_Y0;
    let i = rel / NAV_STEP;
    if i >= 6 || rel % NAV_STEP >= NAV_H {
        return false;
    }
    let views = [VIEW_HOME, VIEW_RECEIVE, VIEW_SEND, VIEW_PROOF, VIEW_SHIELDED, VIEW_NOX];
    state.view = views[i as usize];
    true
}

pub(super) fn hit(x: u32, y: u32, hx: u32, hy: u32, hw: u32, hh: u32) -> bool {
    x >= hx && y >= hy && x < hx + hw && y < hy + hh
}
