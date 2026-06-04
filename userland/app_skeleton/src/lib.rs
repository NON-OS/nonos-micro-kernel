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

#![no_std]

extern crate alloc;

pub mod app;
pub mod clients;
pub mod discover;
pub mod input;
pub mod paint;
pub mod runner;
pub mod setup;
pub mod wire;

pub use app::{App, AppManifest, EventOutcome, WindowKind};
pub use clients::clipboard::{clipboard_copy, clipboard_paste};
pub use input::{
    InputEvent, InputKind, KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC,
    KEY_HOME, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT, KEY_TAB, KEY_UP, MOD_ALT, MOD_CAPS,
    MOD_CTRL, MOD_META, MOD_NUM, MOD_SHIFT,
};
pub use paint::PaintBuffer;
pub use runner::run;
