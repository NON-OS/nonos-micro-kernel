extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use crate::viewer::decode::Decoded;
use crate::viewer::viewport::View;

pub struct ViewerState {
    pub owner_pid: u32,
    pub img: Option<Decoded>,
    pub view: View,
    pub dir: Vec<String>,
    pub idx: usize,
    pub status: String,
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
        }
    }
}
