extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const MAX_TILES: usize = 60;
pub const THUMB_W: u32 = 160;
pub const THUMB_H: u32 = 120;

pub struct Entry {
    pub path: String,
    pub thumb: Option<Vec<u32>>,
    pub tw: u32,
    pub th: u32,
    pub failed: bool,
}

pub struct GalleryState {
    pub entries: Vec<Entry>,
    pub scroll: usize,
    pub sel: usize,
    pub scanned: bool,
}

impl GalleryState {
    pub fn new() -> Self {
        GalleryState { entries: Vec::new(), scroll: 0, sel: 0, scanned: false }
    }
}
