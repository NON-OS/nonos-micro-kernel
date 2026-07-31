extern crate alloc;
use crate::viewer::state::ViewerState;
use crate::viewer::viewport::FitMode;
use alloc::format;
use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;

const PANEL: u32 = 0xC0_10_14_18;
const FG: u32 = 0xFFE6_E6E6;

const KEYMAP: &[(&str, &str)] = &[
    ("< >  <-/->", "prev/next"),
    ("scroll/+/-", "zoom"),
    ("drag", "pan / swipe"),
    ("f/1/w", "fit/actual/fill"),
    ("r/h/v", "rotate/flip-h/flip-v"),
    ("i/?", "info/help"),
    ("space", "slideshow"),
    ("[/]", "slower/faster"),
    ("0", "reset"),
];

fn basename(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

fn ext_upper(path: &str) -> String {
    match path.rfind('.') {
        Some(i) => path[i + 1..].to_ascii_uppercase(),
        None => String::new(),
    }
}

fn mode_name(mode: FitMode) -> &'static str {
    match mode {
        FitMode::Fit => "FIT",
        FitMode::Actual => "ACTUAL",
        FitMode::Fill => "FILL",
    }
}

pub fn draw_info(fb: &mut PaintBuffer, st: &ViewerState) {
    if !st.info_visible {
        return;
    }
    fb.fill_rect(0, 0, 260, 60, PANEL);
    let path = st.dir.get(st.idx).map(|p| p.as_str());
    fb.text(8, 6, path.map(basename).unwrap_or("(no image)").as_bytes(), FG);
    let (w, h) = st.img.as_ref().map(|i| (i.w, i.h)).unwrap_or((0, 0));
    let fmt = path.map(ext_upper).unwrap_or_default();
    fb.text(8, 24, format!("{}x{}  {}  {}B", w, h, fmt, st.file_size).as_bytes(), FG);
    let zoom_pct = (st.view.zoom * 100.0) as u32;
    let line =
        format!("{}/{}  {}%  {}", st.idx + 1, st.dir.len(), zoom_pct, mode_name(st.fit_mode));
    fb.text(8, 42, line.as_bytes(), FG);
}

pub fn draw_help(fb: &mut PaintBuffer, st: &ViewerState) {
    if !st.help_visible {
        return;
    }
    let h = 8 + KEYMAP.len() as u32 * 16;
    fb.fill_rect(0, 0, 260, h, PANEL);
    for (i, (keys, action)) in KEYMAP.iter().enumerate() {
        fb.text(8, 6 + i as u32 * 16, format!("{}  {}", keys, action).as_bytes(), FG);
    }
}

pub fn draw_slideshow(fb: &mut PaintBuffer, st: &ViewerState) {
    if !st.slideshow_on {
        return;
    }
    let line = format!("> {}s", st.interval_ms / 1000);
    fb.text(fb.width.saturating_sub(60), 6, line.as_bytes(), FG);
}
