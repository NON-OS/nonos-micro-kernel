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

mod format_u32;
mod format_u64;
mod hex_hash;
mod logo;
mod logo_bits;
mod paint;
mod paint_account_card;
mod paint_background;
mod paint_button;
mod paint_home;
mod paint_home_activity;
mod paint_network_card;
mod paint_network_labels;
mod paint_home_security;
mod paint_nox;
mod paint_nox_stake;
mod paint_portfolio;
mod paint_proof_view;
mod paint_proofs;
mod paint_rail_card;
mod paint_receive;
mod paint_send;
mod paint_send_side;
mod paint_send_route_label;
mod paint_sidebar;
mod paint_statusbar;
mod paint_statusline;
mod paint_sysbar;
mod paint_topbar;
mod paint_tx;
mod panel;
mod ui;

pub use paint::paint;
// Shared so the pointer handler can hit-test the Generate/probe button at the
// exact rect the Receive screen paints it.
pub use paint_receive::{GEN_BTN_H, GEN_BTN_W, GEN_BTN_X, GEN_BTN_Y};
pub use paint_sidebar::{NAV_H, NAV_STEP, NAV_W, NAV_X, NAV_Y0};
pub use paint_topbar::{THEME_BTN_H, THEME_BTN_W, THEME_BTN_X, THEME_BTN_Y};
