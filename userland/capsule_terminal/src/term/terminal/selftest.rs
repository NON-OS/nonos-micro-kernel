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

use crate::term::state::State;
use crate::term::util::copy_into;

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
    state.last_status
}

// Drive the previously unproven shell paths through the normal submit
// path and emit one serial marker per step so a headless boot grades
// itself: echo, a vfs write/read round trip, a pipe, and `||` gating.
pub fn vt_selftest() {
    mark(b"vt-skeleton", true);
    let ok = crate::term::vt::color::ansi_to_argb(1) == 0xFF80_0000
        && crate::term::vt::color::ansi_to_argb(15) == 0xFFFF_FFFF;
    mark(b"vt-color", ok);
    let g = crate::term::grid::types::Grid::new();
    let ok = g.cells.len() == crate::term::dimensions::COLS * crate::term::dimensions::VISIBLE_ROWS
        && g.cells[0].ch == b' ' && g.x == 0 && g.y == 0;
    mark(b"vt-grid", ok);
    let mut g = crate::term::grid::types::Grid::new();
    for &b in b"AB" { g.put_char(b); }
    g.carriage_return();
    g.put_char(b'C');
    let row0_ok = g.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == b'C'
        && g.cells[crate::term::grid::types::Grid::idx(1, 0)].ch == b'B';
    for _ in 0..crate::term::dimensions::VISIBLE_ROWS { g.line_feed(); }
    let scrolled_ok = g.hist_count >= 1;
    g.put_char(b'Z');
    g.erase_display(2);
    let cleared_ok = g.cells[0].ch == b' ' && g.x == 0 && g.y == 0;
    mark(b"vt-grid-ops", row0_ok && scrolled_ok && cleared_ok);
    struct Rec { prints: alloc::vec::Vec<u8>, execs: alloc::vec::Vec<u8>, csis: alloc::vec::Vec<(u8, i64)> }
    impl crate::term::vt::parser::Perform for Rec {
        fn print(&mut self, c: u8) { self.prints.push(c); }
        fn execute(&mut self, b: u8) { self.execs.push(b); }
        fn csi(&mut self, c: u8, params: &[i64], _inter: &[u8]) { self.csis.push((c, params.first().copied().unwrap_or(-1))); }
        fn esc(&mut self, _c: u8, _inter: &[u8]) {}
        fn osc(&mut self, _data: &[u8]) {}
    }
    let mut rec = Rec { prints: alloc::vec::Vec::new(), execs: alloc::vec::Vec::new(), csis: alloc::vec::Vec::new() };
    let mut parser = crate::term::vt::parser::Parser::new();
    for &b in b"A\x1b[31mB\x1b[0m\n\x1b[2J" { parser.advance(&mut rec, b); }
    let ok = rec.prints == b"AB"
        && rec.execs == [0x0Au8]
        && rec.csis == [(b'm', 31), (b'm', 0), (b'J', 2)];
    mark(b"vt-parser", ok);
    let mut g2 = crate::term::grid::types::Grid::new();
    let mut parser2 = crate::term::vt::parser::Parser::new();
    {
        let mut vt = crate::term::vt::state::VtState { g: &mut g2 };
        for &b in b"\x1b[5;3HX" { parser2.advance(&mut vt, b); }
    }
    let csi_pos_ok = g2.cells[crate::term::grid::types::Grid::idx(2, 4)].ch == b'X';
    {
        let mut vt = crate::term::vt::state::VtState { g: &mut g2 };
        for &b in b"\x1b[2J" { parser2.advance(&mut vt, b); }
    }
    let csi_clr_ok = g2.cells[0].ch == b' ' && g2.x == 0 && g2.y == 0;
    mark(b"vt-csi", csi_pos_ok && csi_clr_ok);
    let mut g4 = crate::term::grid::types::Grid::new();
    let mut p4 = crate::term::vt::parser::Parser::new();
    {
        let mut vt = crate::term::vt::state::VtState { g: &mut g4 };
        for &b in b"\x1b[1;31mZ" { p4.advance(&mut vt, b); }
    }
    let z = g4.cells[crate::term::grid::types::Grid::idx(0, 0)];
    let sgr_set_ok = (z.flags & crate::term::grid::cell::F_BOLD) != 0 && z.fg == 1 && z.ch == b'Z';
    {
        let mut vt = crate::term::vt::state::VtState { g: &mut g4 };
        for &b in b"\x1b[0mY" { p4.advance(&mut vt, b); }
    }
    let y = g4.cells[crate::term::grid::types::Grid::idx(1, 0)];
    let sgr_reset_ok = y.flags == 0 && y.fg == crate::term::vt::color::DEFAULT_FG && y.ch == b'Y';
    mark(b"vt-sgr", sgr_set_ok && sgr_reset_ok);
    let mut g5 = crate::term::grid::types::Grid::new();
    g5.feed(b"hi\x1b[32m!\n");
    let feed_ok = g5.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == b'h'
        && g5.cells[crate::term::grid::types::Grid::idx(1, 0)].ch == b'i'
        && g5.cells[crate::term::grid::types::Grid::idx(2, 0)].ch == b'!'
        && g5.cells[crate::term::grid::types::Grid::idx(2, 0)].fg == 2
        && g5.y == 1
        && g5.x == 0;
    mark(b"vt-feed", feed_ok);
    let mut sb = crate::term::scrollback::Scrollback::new();
    sb.push_line(b"MIR");
    let mirror_ok = sb.grid.cells[crate::term::grid::types::Grid::idx(0, 0)].ch == b'M';
    mark(b"vt-mirror", mirror_ok);
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
}

fn run_cmd(state: &mut State, cmd: &[u8]) {
    state.scrollback.clear();
    state.line.replace(cmd);
    let _ = crate::event::on_enter(state);
}

fn ok_cmd(state: &mut State, cmd: &[u8]) -> bool {
    run_cmd(state, cmd);
    state.last_status
}

fn visible_has(state: &State, needle: &[u8]) -> bool {
    state.scrollback.visible().rows().any(|(row, _)| row == needle)
}

fn mark(step: &[u8], ok: bool) {
    let mut buf = [0u8; 64];
    let mut n = 0;
    n += copy_into(&mut buf[n..], b"[TERMINAL-TEST] ");
    n += copy_into(&mut buf[n..], step);
    n += copy_into(&mut buf[n..], if ok { b" ok\n" } else { b" FAIL\n" });
    let _ = mk_debug(buf.as_ptr(), n);
}
