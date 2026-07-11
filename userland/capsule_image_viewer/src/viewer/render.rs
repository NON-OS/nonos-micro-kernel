use nonos_app_skeleton::PaintBuffer;
use crate::viewer::state::ViewerState;
use crate::viewer::viewport::{place, clamp_pan};
use crate::viewer::scale::{draw_nn, Dst};

const BG: u32 = 0xFF10_1418;
const FG: u32 = 0xFFE6_E6E6;

pub fn paint(st: &mut ViewerState, fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, fb.height, BG);
    match st.img.as_ref() {
        Some(img) => {
            clamp_pan(&mut st.view, img.w, img.h, fb.width, fb.height);
            let p = place(img.w, img.h, fb.width, fb.height, &st.view);
            let (w, h, stride) = (fb.width, fb.height, fb.stride_words);
            let mut dst = Dst { px: fb.pixels, stride, w, h };
            draw_nn(&mut dst, &img.px, img.w, img.h, p.dx, p.dy, p.dw, p.dh);
        }
        None => {}
    }
    if !st.status.is_empty() {
        fb.text(8, fb.height.saturating_sub(20), st.status.as_bytes(), FG);
    }
}
