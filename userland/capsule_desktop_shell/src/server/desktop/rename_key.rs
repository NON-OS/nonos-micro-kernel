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

//! Apply one key press to the in-progress rename: Enter commits, Escape
//! cancels, Backspace deletes, and printable keys are appended.

use crate::state::Context;

const KEY_BACKSPACE: u32 = 0x08;
const KEY_ENTER: u32 = 0x0D;
const KEY_ESC: u32 = 0x1B;
const MAX_NAME: usize = 64;

pub fn rename_key(ctx: &mut Context, code: u32) {
    match code {
        KEY_ENTER => super::commit_rename::commit_rename(ctx),
        KEY_ESC => {
            super::release_keys::release_keys(ctx);
            ctx.rename = None;
            ctx.rename_buf.clear();
        }
        KEY_BACKSPACE => {
            ctx.rename_buf.pop();
        }
        c if (0x20..=0x0010_FFFF).contains(&c) => {
            if ctx.rename_buf.len() < MAX_NAME {
                if let Some(ch) = char::from_u32(c) {
                    ctx.rename_buf.push(ch);
                }
            }
        }
        _ => {}
    }
}
