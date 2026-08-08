extern crate alloc;
use crate::viewer::decode::Decoded;
use crate::viewer::gallery::state::GalleryState;
use crate::viewer::viewport::{FitMode, View};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Gallery,
    Single,
}

pub struct ViewerState {
    pub owner_pid: u32,
    pub img: Option<Decoded>,
    pub view: View,
    pub dir: Vec<String>,
    pub idx: usize,
    pub status: String,
    pub fit_mode: FitMode,
    pub info_visible: bool,
    pub help_visible: bool,
    pub slideshow_on: bool,
    pub interval_ms: u64,
    pub last_advance_ms: u64,
    pub file_size: u64,
    pub dragging: bool,
    pub drag_x: i32,
    pub drag_y: i32,
    pub swipe_start_x: i32,
    pub view_w: u32,
    pub view_h: u32,
    pub mode: Mode,
    pub gallery: GalleryState,
}

impl ViewerState {
    pub fn new() -> Self {
        ViewerState {
            owner_pid: 0,
            img: None,
            view: View { zoom: 1.0, pan_x: 0.0, pan_y: 0.0 },
            dir: Vec::new(),
            idx: 0,
            status: String::new(),
            fit_mode: FitMode::Fit,
            info_visible: false,
            help_visible: false,
            slideshow_on: false,
            interval_ms: 3000,
            last_advance_ms: 0,
            file_size: 0,
            dragging: false,
            drag_x: 0,
            drag_y: 0,
            swipe_start_x: 0,
            view_w: 0,
            view_h: 0,
            mode: Mode::Gallery,
            gallery: GalleryState::new(),
        }
    }
}
