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

mod boot;
mod click_focus;
mod control;
mod decorations;
mod dispatch;
mod drag;
mod drain_ipc;
mod ensure_primed;
#[cfg(feature = "runtime")]
mod entry;
#[cfg(feature = "runtime")]
mod ephemeral;
mod fail;
mod fit_display;
mod frame_finish;
mod idle;
mod maximize;
mod move_window;
mod paint_frame;
mod paint_once;
mod prime_frame;
mod refresh_input;
mod repaint;
mod request_id;
mod resize_window;
mod run_loop;
mod service_frame;
mod teardown;

#[cfg(feature = "runtime")]
pub use entry::run;
pub use run_loop::run_loop;
