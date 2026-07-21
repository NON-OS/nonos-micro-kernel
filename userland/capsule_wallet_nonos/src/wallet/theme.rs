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

use core::sync::atomic::{AtomicBool, Ordering};

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 800;
pub const S1: u32 = 8;
pub const S2: u32 = 16;
pub const S3: u32 = 24;
pub const S4: u32 = 32;
pub const S6: u32 = 48;
pub const CONTENT_X: u32 = 200;
pub const CONTENT_PAD: u32 = 26;

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: u32,
    pub bg2: u32,
    pub line: u32,
    pub line2: u32,
    pub panel: u32,
    pub panel2: u32,
    pub fg: u32,
    pub muted: u32,
    pub dim: u32,
    pub accent: u32,
    pub cyan: u32,
    pub ink: u32,
    pub green: u32,
    pub green_ink: u32,
    pub amber: u32,
    pub amber_ink: u32,
    pub warn: u32,
    pub sel: u32,
    pub sysbar: u32,
    pub neutral_800: u32,
    pub ok: u32,
    pub elev: u32,
    pub accent_dim: u32,
    pub tag_bg: u32,
    pub tag_fg: u32,
    pub neutral_100: u32,
    pub neutral_400: u32,
    pub ok_bg: u32,
    pub warn_bg: u32,
}

const DARK: Palette = Palette {
    bg: 0xFF09_0C12,
    bg2: 0xFF07_0A0F,
    line: 0xFF1A_2230,
    line2: 0xFF26_313F,
    panel: 0xFF0D_1219,
    panel2: 0xFF0A_0F17,
    fg: 0xFFEC_EFF4,
    muted: 0xFF81_8B9A,
    dim: 0xFF56_5F6E,
    accent: 0xFF3D_D9E8,
    cyan: 0xFF3D_D9E8,
    ink: 0xFF04_222A,
    green: 0xFF6F_E6A6,
    green_ink: 0xFF05_271A,
    amber: 0xFFEA_B873,
    amber_ink: 0xFF2B_1E07,
    warn: 0xFFEA_B873,
    sel: 0xFF13_1A25,
    sysbar: 0xFF07_0A0F,
    neutral_800: 0xFF26_313F,
    ok: 0xFF6F_E6A6,
    elev: 0xFF1A_2230,
    accent_dim: 0xFF13_1A25,
    tag_bg: 0xFF0F_3B41,
    tag_fg: 0xFF3D_D9E8,
    neutral_100: 0xFFEC_EFF4,
    neutral_400: 0xFFB2_B6CA,
    ok_bg: 0xFF05_271A,
    warn_bg: 0xFF2B_1E07,
};

const LIGHT: Palette = Palette {
    bg: 0xFFF5_F7FA,
    bg2: 0xFFEC_EFF3,
    line: 0xFFE2_E7ED,
    line2: 0xFFCF_D6DF,
    panel: 0xFFFF_FFFF,
    panel2: 0xFFEE_F1F5,
    fg: 0xFF10_1722,
    muted: 0xFF5C_6672,
    dim: 0xFF97_A0AD,
    accent: 0xFF0F_A5B6,
    cyan: 0xFF0F_A5B6,
    ink: 0xFFFF_FFFF,
    green: 0xFF12_A05C,
    green_ink: 0xFFFF_FFFF,
    amber: 0xFFB9_721A,
    amber_ink: 0xFFFF_FFFF,
    warn: 0xFFB9_721A,
    sel: 0xFFE6_F4F6,
    sysbar: 0xFFEC_EFF3,
    neutral_800: 0xFFCF_D6DF,
    ok: 0xFF12_A05C,
    elev: 0xFFE2_E7ED,
    accent_dim: 0xFFE6_F4F6,
    tag_bg: 0xFFDD_F4F6,
    tag_fg: 0xFF0F_A5B6,
    neutral_100: 0xFF10_1722,
    neutral_400: 0xFF97_A0AD,
    ok_bg: 0xFFFF_FFFF,
    warn_bg: 0xFFFF_FFFF,
};

static LIGHTMODE: AtomicBool = AtomicBool::new(false);

pub fn set_light(v: bool) {
    LIGHTMODE.store(v, Ordering::Relaxed);
}
pub fn is_light() -> bool {
    LIGHTMODE.load(Ordering::Relaxed)
}
fn c() -> Palette {
    if is_light() {
        LIGHT
    } else {
        DARK
    }
}

#[allow(non_snake_case)]
pub fn BG() -> u32 {
    c().bg
}
#[allow(non_snake_case)]
pub fn BG_2() -> u32 {
    c().bg2
}
#[allow(non_snake_case)]
pub fn LINE() -> u32 {
    c().line
}
#[allow(non_snake_case)]
pub fn LINE2() -> u32 {
    c().line2
}
#[allow(non_snake_case)]
pub fn PANEL() -> u32 {
    c().panel
}
#[allow(non_snake_case)]
pub fn PANEL_2() -> u32 {
    c().panel2
}
#[allow(non_snake_case)]
pub fn FG() -> u32 {
    c().fg
}
#[allow(non_snake_case)]
pub fn MUTED() -> u32 {
    c().muted
}
#[allow(non_snake_case)]
pub fn DIM() -> u32 {
    c().dim
}
#[allow(non_snake_case)]
pub fn ACCENT() -> u32 {
    c().accent
}
#[allow(non_snake_case)]
pub fn CYAN() -> u32 {
    c().cyan
}
#[allow(non_snake_case)]
pub fn INK() -> u32 {
    c().ink
}
#[allow(non_snake_case)]
pub fn GREEN() -> u32 {
    c().green
}
#[allow(non_snake_case)]
pub fn GREEN_INK() -> u32 {
    c().green_ink
}
#[allow(non_snake_case)]
pub fn AMBER() -> u32 {
    c().amber
}
#[allow(non_snake_case)]
pub fn AMBER_INK() -> u32 {
    c().amber_ink
}
#[allow(non_snake_case)]
pub fn WARN() -> u32 {
    c().warn
}
#[allow(non_snake_case)]
pub fn SEL() -> u32 {
    c().sel
}
#[allow(non_snake_case)]
pub fn SYSBAR() -> u32 {
    c().sysbar
}
#[allow(non_snake_case)]
pub fn NEUTRAL_800() -> u32 {
    c().neutral_800
}
#[allow(non_snake_case)]
pub fn OK() -> u32 {
    c().ok
}
#[allow(non_snake_case)]
pub fn ELEV_HI() -> u32 {
    c().elev
}
#[allow(non_snake_case)]
pub fn ELEV_LO() -> u32 {
    c().elev
}
#[allow(non_snake_case)]
pub fn ACCENT_DIM() -> u32 {
    c().accent_dim
}
#[allow(non_snake_case)]
pub fn ACCENT_TAG_BG() -> u32 {
    c().tag_bg
}
#[allow(non_snake_case)]
pub fn ACCENT_TAG_FG() -> u32 {
    c().tag_fg
}
#[allow(non_snake_case)]
pub fn NEUTRAL_100() -> u32 {
    c().neutral_100
}
#[allow(non_snake_case)]
pub fn NEUTRAL_400() -> u32 {
    c().neutral_400
}
#[allow(non_snake_case)]
pub fn OK_BG() -> u32 {
    c().ok_bg
}
#[allow(non_snake_case)]
pub fn WARN_BG() -> u32 {
    c().warn_bg
}
