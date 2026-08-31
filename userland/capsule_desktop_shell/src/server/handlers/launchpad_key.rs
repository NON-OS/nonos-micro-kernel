// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Keystrokes typed into the Launchpad search field. The codes and the length
//! cap match the inline-rename field so the shell's two text inputs share one
//! contract.

use crate::render::launchpad::rebuild;
use crate::server::repaint::repaint;
use crate::state::Context;

const KEY_BACKSPACE: u32 = 0x08;
const KEY_ENTER: u32 = 0x0D;
const KEY_ESC: u32 = 0x1B;
const MAX_QUERY: usize = 64;

pub fn key(ctx: &mut Context, code: u32) {
    match code {
        KEY_ESC => {
            if ctx.launchpad_query.is_empty() {
                super::launchpad::close(ctx);
                return;
            }
            ctx.launchpad_query.clear();
        }
        KEY_ENTER => {
            super::launchpad::launch_first(ctx);
            return;
        }
        KEY_BACKSPACE => {
            ctx.launchpad_query.pop();
        }
        c if (0x20..=0x0010_FFFF).contains(&c) => {
            if ctx.launchpad_query.len() >= MAX_QUERY {
                return;
            }
            match char::from_u32(c) {
                Some(ch) => ctx.launchpad_query.push(ch),
                None => return,
            }
        }
        _ => return,
    }
    ctx.launchpad_page = 0;
    rebuild(ctx);
    repaint(ctx);
}
