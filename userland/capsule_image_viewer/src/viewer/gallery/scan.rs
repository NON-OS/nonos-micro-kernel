extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::list_paths;
use crate::viewer::ext::is_codec_image;

pub fn filter_images(paths: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = paths.into_iter().filter(|p| is_codec_image(p.as_bytes())).collect();
    out.sort();
    out
}

pub fn scan(owner_pid: u32) -> Vec<String> {
    filter_images(list_paths(owner_pid, b"/").unwrap_or_default())
}
