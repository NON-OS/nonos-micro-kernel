extern crate alloc;
use crate::viewer::decode::decode;
use crate::viewer::gallery::state::{Entry, GalleryState, MAX_TILES, THUMB_H, THUMB_W};
use crate::viewer::scale::{draw_bilinear, Dst};
use alloc::vec;
use nonos_app_skeleton::clients::vfs::read_file;

const MAX_IMAGE_BYTES: u32 = 16 * 1024 * 1024;

pub fn fit_thumb(sw: u32, sh: u32) -> (u32, u32) {
    if sw == 0 || sh == 0 {
        return (1, 1);
    }
    let sw_s = THUMB_W as f32 / sw as f32;
    let sh_s = THUMB_H as f32 / sh as f32;
    let mut s = if sw_s < sh_s { sw_s } else { sh_s };
    if s > 1.0 {
        s = 1.0;
    }
    (((sw as f32 * s) as u32).max(1), ((sh as f32 * s) as u32).max(1))
}

pub fn decode_next(g: &mut GalleryState, owner_pid: u32) -> bool {
    let n = g.entries.len().min(MAX_TILES);
    let mut i = 0;
    while i < n {
        if g.entries[i].thumb.is_none() && !g.entries[i].failed {
            make_thumb(&mut g.entries[i], owner_pid);
            return true;
        }
        i += 1;
    }
    false
}

fn make_thumb(e: &mut Entry, owner_pid: u32) {
    let bytes = match read_file(owner_pid, e.path.as_bytes(), MAX_IMAGE_BYTES) {
        Ok(b) => b,
        Err(_) => {
            e.failed = true;
            return;
        }
    };
    let d = match decode(&bytes, e.path.as_bytes()) {
        Ok(d) => d,
        Err(_) => {
            e.failed = true;
            return;
        }
    };
    let (tw, th) = fit_thumb(d.w, d.h);
    let mut thumb = vec![0u32; (tw * th) as usize];
    let mut dst = Dst { px: &mut thumb, stride: tw, w: tw, h: th };
    draw_bilinear(&mut dst, &d.px, d.w, d.h, 0, 0, tw, th);
    e.tw = tw;
    e.th = th;
    e.thumb = Some(thumb);
}
