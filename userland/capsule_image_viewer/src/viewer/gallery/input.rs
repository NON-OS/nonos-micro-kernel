extern crate alloc;
use crate::viewer::gallery::layout::{cell_rect, grid, max_scroll};
use crate::viewer::gallery::state::{GalleryState, MAX_TILES};
use alloc::string::String;
use nonos_app_skeleton::input::{KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_RIGHT, KEY_UP};
use nonos_app_skeleton::{InputEvent, InputKind};

pub enum GalleryAction {
    None,
    Open(String),
    Repaint,
}

pub fn hit_tile(x: i32, y: i32, scroll: usize, win_w: u32, count: usize) -> Option<usize> {
    let g = grid(win_w);
    let mut i = 0;
    while i < count {
        let (cx, cy, w, h) = cell_rect(i, scroll, &g);
        if x >= cx && x < cx + w as i32 && y >= cy && y < cy + h as i32 {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub fn on_event(g: &mut GalleryState, ev: &InputEvent, win_w: u32, win_h: u32) -> GalleryAction {
    let count = g.entries.len().min(MAX_TILES);
    match ev.kind {
        InputKind::ButtonDown => match hit_tile(ev.x, ev.y, g.scroll, win_w, count) {
            Some(i) => {
                g.sel = i;
                GalleryAction::Open(g.entries[i].path.clone())
            }
            None => GalleryAction::None,
        },
        InputKind::Wheel => {
            scroll(g, ev.delta_y, win_h, win_w, count);
            GalleryAction::Repaint
        }
        InputKind::KeyDown => key(g, ev.code, win_w, count),
        _ => GalleryAction::None,
    }
}

fn scroll(g: &mut GalleryState, delta_y: i32, win_h: u32, win_w: u32, count: usize) {
    let gr = grid(win_w);
    let max = max_scroll(count, win_h, &gr);
    if delta_y > 0 {
        g.scroll = g.scroll.saturating_sub(1);
    } else {
        g.scroll = (g.scroll + 1).min(max);
    }
}

fn key(g: &mut GalleryState, code: u32, win_w: u32, count: usize) -> GalleryAction {
    if count == 0 {
        return GalleryAction::None;
    }
    let cols = grid(win_w).cols as usize;
    match code {
        KEY_LEFT => {
            g.sel = g.sel.saturating_sub(1);
            GalleryAction::Repaint
        }
        KEY_RIGHT => {
            g.sel = (g.sel + 1).min(count - 1);
            GalleryAction::Repaint
        }
        KEY_UP => {
            g.sel = g.sel.saturating_sub(cols);
            GalleryAction::Repaint
        }
        KEY_DOWN => {
            g.sel = (g.sel + cols).min(count - 1);
            GalleryAction::Repaint
        }
        KEY_ENTER => GalleryAction::Open(g.entries[g.sel].path.clone()),
        _ => GalleryAction::None,
    }
}
