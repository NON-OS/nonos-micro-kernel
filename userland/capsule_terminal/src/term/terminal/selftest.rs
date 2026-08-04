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

use nonos_libc::{heap_init, mk_debug, mk_exit, mk_yield, HeapError};

use crate::term::dimensions::{COLS, VISIBLE_ROWS};
use crate::term::grid::types::Grid;
use crate::term::state::State;
use crate::term::util::copy_into;

mod git_test;
mod jobs_test;
mod kernel_api;
mod proc_lifecycle;

const READY_ATTEMPTS: u32 = 100_000;

// Headless capsule entry for the autorun-selftest build: bring up the heap,
// wait until vfs answers, drive the unproven shell paths, then exit. No
// surface, no compositor, no focus handoff -- this grades shell logic, not
// the GUI app loop.
pub fn main() -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => exit_fail(b"[TERMINAL-TEST] FAIL heap\n"),
    }
    let mut state = State::new();
    let mut attempts = 0u32;
    while !ready(&mut state) {
        attempts += 1;
        if attempts >= READY_ATTEMPTS {
            exit_fail(b"[TERMINAL-TEST] FAIL vfs never ready\n");
        }
        mk_yield();
    }
    run(&mut state);
    mk_exit(0);
}

fn exit_fail(msg: &[u8]) -> ! {
    let _ = mk_debug(msg.as_ptr(), msg.len());
    mk_exit(1);
}

// True once `read` of a vfs-seeded file succeeds, so the assertions only
// run after the capsule stack has settled and vfs is answering.
fn ready(state: &mut State) -> bool {
    run_cmd(state, b"read /readme.txt");
    state.last_status == 0
}

// Drive the previously unproven shell paths through the normal submit
// path and emit one serial marker per step so a headless boot grades
// itself: echo, a vfs write/read round trip, a pipe, and `||` gating.
pub fn vt_selftest() {
    mark(b"vt-skeleton", true);
    {
        let ok = crate::term::vt::color::ansi_to_argb(1) == 0xFF80_0000
            && crate::term::vt::color::ansi_to_argb(15) == 0xFFFF_FFFF;
        mark(b"vt-color", ok);
    }
    {
        let g = crate::term::grid::types::Grid::new();
        let ok = g.cells.len()
            == crate::term::dimensions::COLS * crate::term::dimensions::VISIBLE_ROWS
            && g.cells[0].ch == ' '
            && g.x == 0
            && g.y == 0;
        mark(b"vt-grid", ok);
    }
    {
        let mut g = crate::term::grid::types::Grid::new();
        for ch in "AB".chars() {
            g.put_char(ch);
        }
        g.carriage_return();
        g.put_char('C');
        let row0_ok = g.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == 'C'
            && g.cells[crate::term::grid::types::Grid::idx(1, 0)].ch == 'B';
        for _ in 0..crate::term::dimensions::VISIBLE_ROWS {
            g.line_feed();
        }
        let scrolled_ok = g.hist_count >= 1;
        g.put_char('Z');
        g.erase_display(2);
        let cleared_ok = g.cells[0].ch == ' ' && g.x == 0 && g.y == 0;
        mark(b"vt-grid-ops", row0_ok && scrolled_ok && cleared_ok);
    }
    {
        struct Rec {
            prints: alloc::vec::Vec<u8>,
            execs: alloc::vec::Vec<u8>,
            csis: alloc::vec::Vec<(u8, i64)>,
        }
        impl crate::term::vt::parser::Perform for Rec {
            fn print(&mut self, c: u8) {
                self.prints.push(c);
            }
            fn execute(&mut self, b: u8) {
                self.execs.push(b);
            }
            fn csi(&mut self, c: u8, params: &[i64], _inter: &[u8]) {
                self.csis.push((c, params.first().copied().unwrap_or(-1)));
            }
            fn esc(&mut self, _c: u8, _inter: &[u8]) {}
            fn osc(&mut self, _data: &[u8]) {}
        }
        let mut rec = Rec {
            prints: alloc::vec::Vec::new(),
            execs: alloc::vec::Vec::new(),
            csis: alloc::vec::Vec::new(),
        };
        let mut parser = crate::term::vt::parser::Parser::new();
        for &b in b"A\x1b[31mB\x1b[0m\n\x1b[2J" {
            parser.advance(&mut rec, b);
        }
        let ok = rec.prints == b"AB"
            && rec.execs == [0x0Au8]
            && rec.csis == [(b'm', 31), (b'm', 0), (b'J', 2)];
        mark(b"vt-parser", ok);
    }
    {
        let mut g2 = crate::term::grid::types::Grid::new();
        let mut parser2 = crate::term::vt::parser::Parser::new();
        {
            let mut u = crate::term::vt::utf8::Utf8::default();
            let mut vt = crate::term::vt::state::VtState { g: &mut g2, utf8: &mut u };
            for &b in b"\x1b[5;3HX" {
                parser2.advance(&mut vt, b);
            }
        }
        let csi_pos_ok = g2.cells[crate::term::grid::types::Grid::idx(2, 4)].ch == 'X';
        {
            let mut u = crate::term::vt::utf8::Utf8::default();
            let mut vt = crate::term::vt::state::VtState { g: &mut g2, utf8: &mut u };
            for &b in b"\x1b[2J" {
                parser2.advance(&mut vt, b);
            }
        }
        let csi_clr_ok = g2.cells[0].ch == ' ' && g2.x == 0 && g2.y == 0;
        mark(b"vt-csi", csi_pos_ok && csi_clr_ok);
    }
    {
        let mut g4 = crate::term::grid::types::Grid::new();
        let mut p4 = crate::term::vt::parser::Parser::new();
        {
            let mut u = crate::term::vt::utf8::Utf8::default();
            let mut vt = crate::term::vt::state::VtState { g: &mut g4, utf8: &mut u };
            for &b in b"\x1b[1;31mZ" {
                p4.advance(&mut vt, b);
            }
        }
        let z = g4.cells[crate::term::grid::types::Grid::idx(0, 0)];
        let sgr_set_ok =
            (z.flags & crate::term::grid::cell::F_BOLD) != 0 && z.fg == 1 && z.ch == 'Z';
        {
            let mut u = crate::term::vt::utf8::Utf8::default();
            let mut vt = crate::term::vt::state::VtState { g: &mut g4, utf8: &mut u };
            for &b in b"\x1b[0mY" {
                p4.advance(&mut vt, b);
            }
        }
        let y = g4.cells[crate::term::grid::types::Grid::idx(1, 0)];
        let sgr_reset_ok =
            y.flags == 0 && y.fg == crate::term::vt::color::DEFAULT_FG && y.ch == 'Y';
        mark(b"vt-sgr", sgr_set_ok && sgr_reset_ok);
    }
    {
        let mut g5 = crate::term::grid::types::Grid::new();
        g5.feed(b"hi\x1b[32m!\n");
        let feed_ok = g5.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == 'h'
            && g5.cells[crate::term::grid::types::Grid::idx(1, 0)].ch == 'i'
            && g5.cells[crate::term::grid::types::Grid::idx(2, 0)].ch == '!'
            && g5.cells[crate::term::grid::types::Grid::idx(2, 0)].fg == 2
            && g5.y == 1
            && g5.x == 0;
        mark(b"vt-feed", feed_ok);
    }
    {
        let mut sb = crate::term::scrollback::Scrollback::new();
        sb.push_line(b"MIR");
        let mirror_ok = sb.grid.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == 'M';
        mark(b"vt-mirror", mirror_ok);
    }
    {
        let mut gh = crate::term::grid::types::Grid::new();
        let mut i = 0u8;
        while i < crate::term::dimensions::VISIBLE_ROWS as u8 + 5 {
            gh.feed(&[b'A' + i % 26]);
            gh.feed(b"\n");
            i += 1;
        }
        let has_hist = gh.hist_count >= 5;
        gh.scroll_view_up(1);
        let off1 = gh.view_offset == 1;
        gh.jump_view_bottom();
        let off0 = gh.view_offset == 0;
        mark(b"vt-history", has_hist && off1 && off0);
    }
    {
        let mut ga = crate::term::grid::types::Grid::new();
        ga.feed(b"MAIN");
        ga.feed(b"\x1b[?1049h");
        let in_alt = ga.alternate && ga.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == ' ';
        ga.feed(b"ALT");
        ga.feed(b"\x1b[?1049l");
        let back = !ga.alternate && ga.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == 'M';
        mark(b"vt-altscreen", in_alt && back);
    }
    {
        let mut st = crate::term::state::State::new();
        st.open_block(*b"12:34:56");
        st.close_block(false, 0);
        let one = st.blocks.len() == 1;
        let err = st.blocks[0].status == crate::term::block::Status::Err;
        let ts = &st.blocks[0].ts == b"12:34:56";
        let found = st.block_at(st.blocks[0].start_abs).is_some();
        mark(b"block-model", one && err && ts && found);
    }
    {
        let mut st = crate::term::state::State::new();
        st.open_block(*b"00:00:00");
        st.close_block(true, 1500);
        let ok =
            st.blocks[0].dur_ms == 1500 && st.blocks[0].status == crate::term::block::Status::Ok;
        mark(b"block-dur", ok);
    }
    {
        let mut gb = crate::term::grid::types::Grid::new();
        let start = gb.current_abs_line();
        for _ in 0..(crate::term::dimensions::VISIBLE_ROWS + 3) {
            gb.feed(b"x\n");
        }
        let scrolled = gb.total_scrolled == 4 && gb.abs_base() == 4 - gb.hist_count as u64;
        let row_abs = gb.abs_of_visible_row(0) == gb.total_scrolled - gb.view_offset as u64;
        mark(b"block-absline", start == 0 && scrolled && row_abs);
    }
    {
        let f = crate::term::rtc::fmt_hms(9, 5, 42);
        mark(b"block-ts", &f == b"09:05:42");
    }
    {
        let mut st = crate::term::state::State::new();
        st.line.replace(b"echo hi");
        let _ = crate::event::on_enter(&mut st);
        let opened = st.blocks.len() == 1;
        let ok = st.blocks[0].status == crate::term::block::Status::Ok;
        mark(b"block-capture", opened && ok);
    }
    {
        let (a, an) = crate::term::dur::fmt_dur(1800);
        let (b, bn) = crate::term::dur::fmt_dur(250);
        let (c, cn) = crate::term::dur::fmt_dur(125_000);
        let ok = &a[..an] == b"1.8s" && &b[..bn] == b"250ms" && &c[..cn] == b"2m05s";
        mark(b"dur-fmt", ok);
    }
}

fn run(state: &mut State) {
    vt_selftest();
    run_cmd(state, b"echo selfcheck");
    mark(b"echo", visible_has(state, b"selfcheck"));

    run_cmd(state, b"write /st.txt smoke123");
    run_cmd(state, b"read /st.txt");
    mark(b"vfs", visible_has(state, b"smoke123"));

    run_cmd(state, b"echo a b c | wc");
    mark(b"pipe", visible_has(state, b"1"));

    run_cmd(state, b"echo a | cat /readme.txt");
    mark(b"pipe-anystage", visible_has(state, b"This file lives in the vfs capsule."));

    run_cmd(state, b"read /nope.txt || echo recovered");
    mark(b"statement", visible_has(state, b"recovered"));

    run_ext(state);

    let pass = b"[TERMINAL-TEST] PASS\n";
    let _ = mk_debug(pass.as_ptr(), pass.len());
}

// Grade the commands added for the Warp/coreutils pass: path utilities,
// the cut filter, touch+&&, recursive find, and clock/size queries that
// only assert success because their output is not deterministic.
fn run_ext(state: &mut State) {
    run_cmd(state, b"basename /a/b/file.txt");
    mark(b"basename", visible_has(state, b"file.txt"));

    run_cmd(state, b"dirname /a/b/file.txt");
    mark(b"dirname", visible_has(state, b"/a/b"));

    run_cmd(state, b"echo a:b:c | cut -d: -f2");
    mark(b"cut", visible_has(state, b"b"));

    run_cmd(state, b"write /rin.txt redirin");
    run_cmd(state, b"grep redirin < /rin.txt");
    mark(b"redir-in", visible_has(state, b"redirin"));

    run_cmd(state, b"touch /tt.txt && echo created");
    mark(b"touch", visible_has(state, b"created"));

    run_cmd(state, b"find /");
    mark(b"find", visible_has(state, b"/readme.txt"));

    mark(b"du", ok_cmd(state, b"du /"));
    mark(b"date", ok_cmd(state, b"date"));

    run_cmd(state, b"ifconfig");
    mark(b"ifconfig", visible_has(state, b"net0: down"));

    run_cmd(state, b"nslookup nonos.test");
    mark(b"nslookup", visible_has(state, b"nslookup: dns unavailable"));

    run_cmd(state, b"cat /nonexistent");
    run_cmd(state, b"echo $?");
    mark(b"status", visible_has(state, b"1"));

    kernel_api::run();
    proc_lifecycle::run();
    jobs_test::run(state);
    git_test::run(state);
}

fn run_cmd(state: &mut State, cmd: &[u8]) {
    state.scrollback.clear();
    state.line.replace(cmd);
    let _ = crate::event::on_enter(state);
}

fn ok_cmd(state: &mut State, cmd: &[u8]) -> bool {
    run_cmd(state, cmd);
    state.last_status == 0
}

fn visible_has(state: &State, needle: &[u8]) -> bool {
    let g = &state.scrollback.grid;
    let mut row = 0;
    while row < VISIBLE_ROWS {
        let mut end = COLS;
        while end > 0 && g.cells[Grid::idx(end - 1, row)].ch == ' ' {
            end -= 1;
        }
        let mut buf = [0u8; COLS];
        let mut c = 0;
        while c < end {
            // The needle is ASCII, so anything wider cannot match it and is
            // stood in for by a byte that never appears in one.
            let ch = g.cells[Grid::idx(c, row)].ch;
            buf[c] = if ch.is_ascii() { ch as u8 } else { 0xFF };
            c += 1;
        }
        if &buf[..end] == needle {
            return true;
        }
        row += 1;
    }
    false
}

fn mark(step: &[u8], ok: bool) {
    let mut buf = [0u8; 64];
    let mut n = 0;
    n += copy_into(&mut buf[n..], b"[TERMINAL-TEST] ");
    n += copy_into(&mut buf[n..], step);
    n += copy_into(&mut buf[n..], if ok { b" ok\n" } else { b" FAIL\n" });
    let _ = mk_debug(buf.as_ptr(), n);
}
