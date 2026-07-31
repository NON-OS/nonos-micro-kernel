extern crate alloc;
use crate::viewer::state::ViewerState;
use crate::viewer::viewport::View;
use crate::viewer::{decode, ext, flip, rotate};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::{list_paths, read_file};

const MAX_IMAGE_BYTES: u32 = 16 * 1024 * 1024;

pub fn open_path(st: &mut ViewerState, path: &str) {
    st.view = View { zoom: 1.0, pan_x: 0.0, pan_y: 0.0 };
    let bytes = match read_with_retry(st.owner_pid, path.as_bytes(), MAX_IMAGE_BYTES) {
        Ok(b) => b,
        Err(e) => {
            st.img = None;
            st.status = err_line(path, e);
            return;
        }
    };
    st.file_size = bytes.len() as u64;
    match decode::decode(&bytes, path.as_bytes()) {
        Ok(d) => {
            st.img = Some(d);
            st.status = String::new();
            build_dir(st, path);
        }
        Err(e) => {
            st.img = None;
            st.status = err_line(path, e);
        }
    }
}

fn read_with_retry(owner_pid: u32, path: &[u8], max: u32) -> Result<Vec<u8>, &'static str> {
    let mut attempt = 0u32;
    loop {
        match read_file(owner_pid, path, max) {
            Ok(b) => return Ok(b),
            Err(e) => {
                attempt += 1;
                if attempt >= 8 || e != "vfs ipc failed" {
                    return Err(e);
                }
                for _ in 0..48 {
                    let _ = nonos_libc::mk_yield();
                }
            }
        }
    }
}

pub fn step(st: &mut ViewerState, delta: i32) {
    if st.dir.is_empty() {
        return;
    }
    let n = st.dir.len() as i32;
    st.idx = (((st.idx as i32 + delta) % n + n) % n) as usize;
    let p = st.dir[st.idx].clone();
    open_path(st, &p);
}

pub fn rotate(st: &mut ViewerState) {
    if let Some(img) = st.img.as_mut() {
        let (px, w, h) = rotate::rotate_cw(&img.px, img.w, img.h);
        img.px = px;
        img.w = w;
        img.h = h;
    }
}

pub fn flip_h(st: &mut ViewerState) {
    if let Some(img) = st.img.as_mut() {
        img.px = flip::flip_h(&img.px, img.w, img.h);
    }
}

pub fn flip_v(st: &mut ViewerState) {
    if let Some(img) = st.img.as_mut() {
        img.px = flip::flip_v(&img.px, img.w, img.h);
    }
}

fn build_dir(st: &mut ViewerState, path: &str) {
    let (dir, _file) = split_parent(path);
    let paths = list_paths(st.owner_pid, dir.as_bytes()).unwrap_or_default();
    let mut imgs: Vec<String> = Vec::new();
    for p in paths {
        let Some(rest) = p.strip_prefix(dir.as_str()) else { continue };
        if rest.is_empty() || rest.contains('/') {
            continue;
        }
        if ext::is_codec_image(p.as_bytes()) {
            imgs.push(p);
        }
    }
    imgs.sort();
    st.idx = imgs.iter().position(|p| p == path).unwrap_or(0);
    st.dir = imgs;
}

fn split_parent(path: &str) -> (String, String) {
    match path.rfind('/') {
        Some(i) => (path[..=i].to_string(), path[i + 1..].to_string()),
        None => ("/".to_string(), path.to_string()),
    }
}

fn err_line(path: &str, e: &str) -> String {
    let mut s = String::from("Can't display ");
    s.push_str(path);
    s.push_str(": ");
    s.push_str(e);
    s
}
