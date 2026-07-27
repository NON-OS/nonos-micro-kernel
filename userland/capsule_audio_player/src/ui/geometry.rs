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

pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x as i32
            && px < (self.x + self.w) as i32
            && py >= self.y as i32
            && py < (self.y + self.h) as i32
    }
}

pub struct Layout {
    pub header: Rect,
    pub logo: Rect,
    pub now_tab: Rect,
    pub lib_tab: Rect,
    pub list: Rect,
    pub panel: Rect,
    pub art: Rect,
    pub badge: Rect,
    pub title: Rect,
    pub artist: Rect,
    pub waveform: Rect,
    pub vu: Rect,
    pub shuffle: Rect,
    pub prev: Rect,
    pub play: Rect,
    pub next: Rect,
    pub repeat: Rect,
    pub speaker: Rect,
    pub volume: Rect,
}

fn vc(y: u32, h: u32, ih: u32) -> u32 {
    y + h.saturating_sub(ih) / 2
}

pub fn list_row_h(list: &Rect) -> u32 {
    (list.h / 6).max(44)
}

pub fn list_row_at(list: &Rect, count: usize, x: i32, y: i32) -> Option<usize> {
    if !list.contains(x, y) {
        return None;
    }
    let rh = list_row_h(list);
    let top = list.y + rh;
    if (y as u32) < top {
        return None;
    }
    let idx = ((y as u32 - top) / rh) as usize;
    if top + (idx as u32 + 1) * rh > list.y + list.h {
        return None;
    }
    if idx < count { Some(idx) } else { None }
}

pub fn layout(width: u32, height: u32) -> Layout {
    let pad = (width / 24).max(14);
    let gap = (width / 26).max(10);
    let cw = width.saturating_sub(pad * 2);
    let head_h = (height / 8).max(40);
    let head_y = pad * 3 / 4;
    let logo = head_h * 62 / 100;
    let tab_w = (width * 17 / 100).max(120);
    let tab_h = head_h * 6 / 10;
    let lib_x = width.saturating_sub(pad + tab_w);
    let now_x = lib_x.saturating_sub(tab_w + 8);
    let tr_h = (height * 16 / 100).max(58);
    let tr_y = height.saturating_sub(pad + tr_h);
    let panel_y = head_y + head_h + height * 4 / 100;
    let panel_h = tr_y.saturating_sub(panel_y + height * 4 / 100);
    let ip = pad;
    let art_s = panel_h.saturating_sub(ip * 2).min(width * 26 / 100);
    let vu_w = (width * 3 / 100).max(10);
    let vu_x = pad + cw.saturating_sub(ip + vu_w);
    let cx = pad + ip + art_s + gap;
    let cw2 = vu_x.saturating_sub(cx + gap);
    let badge_y = panel_y + ip + panel_h / 12;
    let title_y = badge_y + head_h * 55 / 100;
    let artist_y = title_y + head_h * 62 / 100;
    let time_h = tr_h * 30 / 100;
    let wf_h = (panel_h * 22 / 100).max(40);
    let wf_y = (panel_y + panel_h).saturating_sub(ip + wf_h + time_h);
    let icon = (tr_h * 40 / 100).max(18);
    let play_s = (tr_h * 92 / 100).max(52);
    let gt = (width * 3 / 100).max(12);
    let pv_x = pad + icon + gt;
    let pl_x = pv_x + icon + gt;
    let nx_x = pl_x + play_s + gt;
    let rp_x = nx_x + icon + gt;
    let vol_w = (width * 26 / 100).max(120);
    let spk_x = width.saturating_sub(pad + vol_w);
    let slider_x = spk_x + icon + gap;
    let slider_w = width.saturating_sub(pad + slider_x);
    let sl_h = tr_h * 20 / 100;
    Layout {
        header: Rect { x: pad, y: head_y, w: cw, h: head_h },
        logo: Rect { x: pad, y: vc(head_y, head_h, logo), w: logo, h: logo },
        now_tab: Rect { x: now_x, y: vc(head_y, head_h, tab_h), w: tab_w, h: tab_h },
        lib_tab: Rect { x: lib_x, y: vc(head_y, head_h, tab_h), w: tab_w, h: tab_h },
        list: Rect { x: pad, y: panel_y, w: cw, h: height.saturating_sub(panel_y + pad) },
        panel: Rect { x: pad, y: panel_y, w: cw, h: panel_h },
        art: Rect { x: pad + ip, y: panel_y + panel_h.saturating_sub(art_s) / 2, w: art_s, h: art_s },
        badge: Rect { x: cx, y: badge_y, w: cw2, h: head_h / 3 },
        title: Rect { x: cx, y: title_y, w: cw2, h: head_h * 55 / 100 },
        artist: Rect { x: cx, y: artist_y, w: cw2, h: head_h / 2 },
        waveform: Rect { x: cx, y: wf_y, w: cw2, h: wf_h },
        vu: Rect { x: vu_x, y: panel_y + ip, w: vu_w, h: panel_h.saturating_sub(ip * 2) },
        shuffle: Rect { x: pad, y: vc(tr_y, tr_h, icon), w: icon, h: icon },
        prev: Rect { x: pv_x, y: vc(tr_y, tr_h, icon), w: icon, h: icon },
        play: Rect { x: pl_x, y: vc(tr_y, tr_h, play_s), w: play_s, h: play_s },
        next: Rect { x: nx_x, y: vc(tr_y, tr_h, icon), w: icon, h: icon },
        repeat: Rect { x: rp_x, y: vc(tr_y, tr_h, icon), w: icon, h: icon },
        speaker: Rect { x: spk_x, y: vc(tr_y, tr_h, icon), w: icon, h: icon },
        volume: Rect { x: slider_x, y: vc(tr_y, tr_h, sl_h), w: slider_w, h: sl_h },
    }
}
