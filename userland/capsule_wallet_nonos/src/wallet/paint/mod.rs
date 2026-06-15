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
mod paint_network_card;
mod paint_network_labels;
mod paint_home_security;
mod paint_proof_view;
mod paint_proofs;
mod paint_rail_card;
mod paint_receive;
mod paint_send;
mod paint_send_route_label;
mod paint_sidebar;
mod paint_statusbar;
mod paint_topbar;
mod paint_tx;
mod panel;

pub use paint::paint;
