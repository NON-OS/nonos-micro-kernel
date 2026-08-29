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

//! The editor palette. A theme carries every colour the shell paints with, so
//! switching themes recolours the whole editor at once: body, gutter, syntax,
//! and the chrome (tab bar, sidebar, activity bar, status bar). One theme is
//! active process-wide, held in an atomic so every tab shares it and the theme
//! button can flip it from any paint path.

use core::sync::atomic::{AtomicUsize, Ordering};

// A full palette. Every field is an ARGB colour with a 0xFF alpha.
#[derive(Clone, Copy)]
pub struct Theme {
    pub background: u32,
    pub foreground: u32,
    pub title: u32,
    pub muted: u32,
    pub header_bg: u32,
    pub activity_bg: u32,
    pub sidebar_bg: u32,
    pub tabbar_bg: u32,
    pub tab_active_bg: u32,
    pub tab_inactive_bg: u32,
    pub row_select: u32,
    pub icon: u32,
    pub icon_active: u32,
    pub folder: u32,
    pub gutter_bg: u32,
    pub gutter_fg: u32,
    pub gutter_cur: u32,
    pub line: u32,
    pub accent: u32,
    pub caret: u32,
    pub current_line: u32,
    pub selection: u32,
    pub syn_keyword: u32,
    pub syn_string: u32,
    pub syn_comment: u32,
    pub syn_number: u32,
    pub syn_type: u32,
    pub syn_function: u32,
}

// The bundled themes, cycled by the theme button and Ctrl+B. Index 0 is the
// NONOS Docs default: cyan on deep navy, matching the shell's own identity.
pub const THEMES: [Theme; 4] = [
    // NONOS Docs.
    Theme {
        background: 0xFF060C15,
        foreground: 0xFFE4ECF5,
        title: 0xFFE4ECF5,
        muted: 0xFF9BB0C7,
        header_bg: 0xFF08111D,
        activity_bg: 0xFF08111D,
        sidebar_bg: 0xFF08111D,
        tabbar_bg: 0xFF08111D,
        tab_active_bg: 0xFF060C15,
        tab_inactive_bg: 0xFF0E1B2C,
        row_select: 0xFF132539,
        icon: 0xFF61788F,
        icon_active: 0xFFE4ECF5,
        folder: 0xFF17BED9,
        gutter_bg: 0xFF060C15,
        gutter_fg: 0xFF456079,
        gutter_cur: 0xFFE4ECF5,
        line: 0xFF1C2F47,
        accent: 0xFF17BED9,
        caret: 0xFF17BED9,
        current_line: 0xFF0B1524,
        selection: 0xFF0C4C5D,
        syn_keyword: 0xFFC678DD,
        syn_string: 0xFF98C379,
        syn_comment: 0xFF5C6370,
        syn_number: 0xFFD19A66,
        syn_type: 0xFFE5C07B,
        syn_function: 0xFF61AFEF,
    },
    // Light.
    Theme {
        background: 0xFFFAFAFA,
        foreground: 0xFF383A42,
        title: 0xFF202020,
        muted: 0xFF808080,
        header_bg: 0xFFEAEAEB,
        activity_bg: 0xFFE0E0E1,
        sidebar_bg: 0xFFEAEAEB,
        tabbar_bg: 0xFFEAEAEB,
        tab_active_bg: 0xFFFAFAFA,
        tab_inactive_bg: 0xFFECECEC,
        row_select: 0xFFDDDDDD,
        icon: 0xFF808080,
        icon_active: 0xFF202020,
        folder: 0xFF4078F2,
        gutter_bg: 0xFFFAFAFA,
        gutter_fg: 0xFFB0B0B0,
        gutter_cur: 0xFF383A42,
        line: 0xFFD0D0D0,
        accent: 0xFF4078F2,
        caret: 0xFF526EFF,
        current_line: 0xFFF0F0F0,
        selection: 0xFFCFD8E6,
        syn_keyword: 0xFFA626A4,
        syn_string: 0xFF50A14F,
        syn_comment: 0xFFA0A1A7,
        syn_number: 0xFF986801,
        syn_type: 0xFFC18401,
        syn_function: 0xFF4078F2,
    },
    // Dracula.
    Theme {
        background: 0xFF282A36,
        foreground: 0xFFF8F8F2,
        title: 0xFFF8F8F2,
        muted: 0xFF6272A4,
        header_bg: 0xFF21222C,
        activity_bg: 0xFF191A21,
        sidebar_bg: 0xFF21222C,
        tabbar_bg: 0xFF21222C,
        tab_active_bg: 0xFF282A36,
        tab_inactive_bg: 0xFF21222C,
        row_select: 0xFF44475A,
        icon: 0xFF6272A4,
        icon_active: 0xFFF8F8F2,
        folder: 0xFFBD93F9,
        gutter_bg: 0xFF282A36,
        gutter_fg: 0xFF6272A4,
        gutter_cur: 0xFFF8F8F2,
        line: 0xFF44475A,
        accent: 0xFFBD93F9,
        caret: 0xFFF8F8F0,
        current_line: 0xFF44475A,
        selection: 0xFF44475A,
        syn_keyword: 0xFFFF79C6,
        syn_string: 0xFFF1FA8C,
        syn_comment: 0xFF6272A4,
        syn_number: 0xFFBD93F9,
        syn_type: 0xFF8BE9FD,
        syn_function: 0xFF50FA7B,
    },
    // Nord.
    Theme {
        background: 0xFF2E3440,
        foreground: 0xFFD8DEE9,
        title: 0xFFECEFF4,
        muted: 0xFF616E88,
        header_bg: 0xFF2E3440,
        activity_bg: 0xFF272C36,
        sidebar_bg: 0xFF2E3440,
        tabbar_bg: 0xFF2E3440,
        tab_active_bg: 0xFF3B4252,
        tab_inactive_bg: 0xFF2E3440,
        row_select: 0xFF434C5E,
        icon: 0xFF616E88,
        icon_active: 0xFFD8DEE9,
        folder: 0xFF88C0D0,
        gutter_bg: 0xFF2E3440,
        gutter_fg: 0xFF4C566A,
        gutter_cur: 0xFFD8DEE9,
        line: 0xFF434C5E,
        accent: 0xFF88C0D0,
        caret: 0xFFD8DEE9,
        current_line: 0xFF3B4252,
        selection: 0xFF434C5E,
        syn_keyword: 0xFF81A1C1,
        syn_string: 0xFFA3BE8C,
        syn_comment: 0xFF616E88,
        syn_number: 0xFFB48EAD,
        syn_type: 0xFF8FBCBB,
        syn_function: 0xFF88C0D0,
    },
];

// The index of the active theme, shared by every tab and paint path.
static ACTIVE: AtomicUsize = AtomicUsize::new(0);

// The palette every painter reads from this frame.
pub fn active() -> &'static Theme {
    &THEMES[ACTIVE.load(Ordering::Relaxed) % THEMES.len()]
}

// Advance to the next theme, wrapping. Driven by the theme button and Ctrl+B.
pub fn cycle() {
    ACTIVE.store((ACTIVE.load(Ordering::Relaxed) + 1) % THEMES.len(), Ordering::Relaxed);
}
