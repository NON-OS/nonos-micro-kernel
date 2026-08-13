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

use crate::protocol::{read_i32, read_u16, read_u32};
use crate::render::layout::{bottom_dock_rect, menubar_height};
use crate::render::{desktop_icons, desktop_menu, topbar};
use crate::server::desktop;
use crate::server::handlers::{launcher_focus, launcher_request, launchpad};
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::{reveal_taskbar, Context, LAUNCHER_APPS};
use nonos_libc::{
    mk_time_millis, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN,
    INPUT_KIND_POINTER_ABS, INPUT_KIND_TOUCH,
};

const HOVER_REVEAL_BAND: u32 = 4;
/// Pointer travel, in pixels, before a press on an icon becomes a drag.
const DRAG_THRESHOLD: u32 = 6;

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    if buf.len() < 40 || read_u32(buf, 0) != Some(0x4E49_4E50) {
        return false;
    }
    if read_u16(buf, 4) != Some(1) {
        return true;
    }
    let Some(kind) = read_u16(buf, 8) else {
        return true;
    };
    // A key press while an inline rename is active edits the name; keys carry no
    // pointer position, so this is handled before the x/y checks below.
    if kind == INPUT_KIND_KEY_DOWN {
        if ctx.rename.is_some() {
            let code = read_u32(buf, 12).unwrap_or(0);
            desktop::rename_key(ctx, code);
            super::repaint::repaint(ctx);
        }
        return true;
    }
    let Some(x) = read_i32(buf, 16) else {
        return true;
    };
    let Some(y) = read_i32(buf, 20) else {
        return true;
    };
    // The release of a drag must never be missed, or the icon stays glued to
    // the cursor; resolve it before the coordinate bail-out below (the drop
    // uses the last tracked drag position, not this event's).
    if kind == INPUT_KIND_BUTTON_UP && ctx.drag_from.is_some() {
        drop_drag(ctx);
        return true;
    }
    if x < 0 || y < 0 {
        return true;
    }
    if kind == INPUT_KIND_POINTER_ABS {
        // An icon drag in progress follows the pointer once it clears the
        // threshold, and takes priority over menu hover and dock reveal.
        if ctx.drag_from.is_some() {
            let (nx, ny) = (x as u32, y as u32);
            if ctx.drag_moved {
                ctx.drag_x = nx;
                ctx.drag_y = ny;
                super::repaint::repaint(ctx);
            } else if nx.abs_diff(ctx.drag_x) > DRAG_THRESHOLD
                || ny.abs_diff(ctx.drag_y) > DRAG_THRESHOLD
            {
                ctx.drag_moved = true;
                ctx.drag_x = nx;
                ctx.drag_y = ny;
                super::repaint::repaint(ctx);
            }
            return true;
        }
        // With the menu open, track which row the pointer is over so it lights
        // up; otherwise fall through to the dock reveal behaviour.
        if ctx.desktop_menu.is_some() {
            let hovered = desktop_menu::hit(ctx, x as u32, y as u32);
            if hovered != ctx.menu_hover {
                ctx.menu_hover = hovered;
                super::repaint::repaint(ctx);
            }
            return true;
        }
        hover_reveal(ctx, y as u32);
        return true;
    }
    if kind == INPUT_KIND_BUTTON_UP {
        return true;
    }
    if !matches!(kind, INPUT_KIND_TOUCH | INPUT_KIND_BUTTON_DOWN) {
        return true;
    }
    let (px, py) = (x as u32, y as u32);
    if super::handlers::consent::click(ctx, px, py) {
        return true;
    }
    if super::handlers::pkg_consent::click(ctx, px, py) {
        return true;
    }
    // The Launchpad, while open, captures every click: a tile launches its app
    // or tool, and anything else dismisses the overlay.
    if ctx.launchpad {
        launchpad::click(ctx, px, py);
        return true;
    }
    // While the right-click menu is open, the next click either picks an item
    // or dismisses it. Handle that before anything else consumes the click.
    if ctx.desktop_menu.is_some() {
        let row = desktop_menu::hit(ctx, px, py);
        match (ctx.menu_target, row) {
            // Empty-desktop menu.
            (None, Some(0)) => desktop::create_entry(ctx, false),
            (None, Some(1)) => desktop::create_entry(ctx, true),
            // Per-item menu: Open / Rename / Delete.
            (Some(idx), Some(0)) => open_item(ctx, idx),
            (Some(idx), Some(1)) => desktop::start_rename(ctx, idx),
            (Some(idx), Some(2)) => desktop::delete_entry(ctx, idx),
            _ => {}
        }
        ctx.desktop_menu = None;
        ctx.menu_hover = None;
        ctx.menu_target = None;
        super::repaint::repaint(ctx);
        return true;
    }
    // Right-click (button code 2) on the desktop opens a menu: New Folder /
    // New File on empty space, or Open / Rename / Delete on an icon.
    if kind == INPUT_KIND_BUTTON_DOWN && read_u32(buf, 12) == Some(2) {
        let dock_top = bottom_dock_rect(ctx.width, ctx.height).y.saturating_sub(18);
        if py > menubar_height() && py < dock_top {
            ctx.menu_target = desktop_icons::hit(ctx, px, py);
            ctx.desktop_menu = Some((px, py));
            ctx.menu_hover = None;
            super::repaint::repaint(ctx);
        }
        return true;
    }
    // Clicking the brand on the menu bar brings up the app dock.
    if topbar::brand_hit(px, py) {
        if !ctx.taskbar.visible {
            reveal_taskbar(&mut ctx.taskbar, mk_time_millis());
            refresh_taskbar(ctx);
        }
        return true;
    }
    // A left-press on an icon begins a potential drag. If the pointer never
    // moves, release opens it; if it moves onto a folder, release moves it
    // there. Both resolve in drop_drag. The pointer is grabbed for the whole
    // gesture so the release reaches the shell even over another window.
    if let Some(idx) = desktop_icons::hit(ctx, px, py) {
        ctx.drag_from = Some(idx);
        ctx.drag_moved = false;
        ctx.drag_x = px;
        ctx.drag_y = py;
        desktop::grab_drag(ctx);
        return true;
    }
    if !ctx.taskbar.visible {
        let dock = bottom_dock_rect(ctx.width, ctx.height);
        if y as u32 >= dock.y.saturating_sub(18) {
            reveal_taskbar(&mut ctx.taskbar, mk_time_millis());
            refresh_taskbar(ctx);
        }
        return true;
    }
    launcher_focus::handle(ctx, x as u32, y as u32);
    true
}

// Resolve a drag on release: dropped onto a folder it moves the file there, a
// press that never moved is a click that opens the item. Always lets go of the
// pointer grab and the drag state, so nothing stays stuck to the cursor.
fn drop_drag(ctx: &mut Context) {
    let Some(src) = ctx.drag_from.take() else {
        return;
    };
    desktop::release_drag(ctx);
    if ctx.drag_moved {
        if let Some(target) = desktop_icons::hit(ctx, ctx.drag_x, ctx.drag_y) {
            desktop::move_into(ctx, src, target);
        }
    } else {
        open_item(ctx, src);
    }
    ctx.drag_moved = false;
    super::repaint::repaint(ctx);
}

fn is_image_name(name: &str) -> bool {
    let Some((_, ext)) = name.rsplit_once('.') else { return false };
    ext.eq_ignore_ascii_case("png")
        || ext.eq_ignore_ascii_case("jpg")
        || ext.eq_ignore_ascii_case("jpeg")
        || ext.eq_ignore_ascii_case("bmp")
}

// Open a desktop item in the app that suits it, and tell that app which item.
// This used to launch the file manager or the text editor and hand over
// nothing, so every icon opened the same blank app at its default location.
// The path travels the way the file manager's Open With already sends it.
fn open_item(ctx: &mut Context, idx: usize) {
    let Some(item) = ctx.desktop_items.get(idx) else { return };
    // An image has no viewer in this image, and the editor would show a
    // screenful of bytes rather than a picture, so nothing opens.
    if !item.is_dir && is_image_name(&item.name) {
        return;
    }
    let service: &[u8] = if item.is_dir { b"app.file_manager" } else { b"app.text_editor" };
    // By service rather than by position: the table is edited often enough
    // that an index would drift into launching the wrong app.
    let Some(app) = LAUNCHER_APPS.iter().find(|a| a.service == service) else { return };
    // Desktop items are the root listing, so the path is the name under "/".
    let mut path = alloc::string::String::from("/");
    path.push_str(&item.name);
    if let Ok(service) = core::str::from_utf8(app.service) {
        ctx.pending_open.insert(alloc::string::String::from(service), path);
    }
    launcher_request::request(app);
}

fn hover_reveal(ctx: &mut Context, y: u32) {
    if ctx.height == 0 {
        return;
    }
    if !ctx.taskbar.visible {
        if y >= ctx.height.saturating_sub(HOVER_REVEAL_BAND) {
            reveal_taskbar(&mut ctx.taskbar, mk_time_millis());
            refresh_taskbar(ctx);
        }
        return;
    }
    if y >= bottom_dock_rect(ctx.width, ctx.height).y {
        reveal_taskbar(&mut ctx.taskbar, mk_time_millis());
    }
    // No auto-collapse: the dock stays visible as a persistent taskbar.
}
