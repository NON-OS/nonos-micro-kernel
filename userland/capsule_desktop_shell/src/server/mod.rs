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

pub mod desktop;
mod dispatch;
mod frametime;
pub mod handlers;
mod input;
mod installed_apps;
mod packages;
mod paint_initial;
mod ready_to_block;
mod refresh_taskbar;
mod repaint;
pub mod respond;
mod retry_input_subscription;
mod retry_wm_subscription;
pub mod runner;
mod store_health;
mod wm_notify;
mod wm_notify_app_index;
mod wm_notify_label;
mod wm_notify_toast;

pub use runner::run;
