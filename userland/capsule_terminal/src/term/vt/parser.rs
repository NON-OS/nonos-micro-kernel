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

use alloc::vec::Vec;

pub trait Perform {
    fn print(&mut self, c: u8);
    fn execute(&mut self, b: u8);
    fn csi(&mut self, c: u8, params: &[i64], inter: &[u8]);
    fn esc(&mut self, c: u8, inter: &[u8]);
    fn osc(&mut self, data: &[u8]);
}

#[derive(Clone, Copy, PartialEq)]
enum PState {
    Ground,
    Escape,
    EscapeInter,
    CsiEntry,
    CsiParam,
    CsiInter,
    Osc,
    OscEsc,
}

pub struct Parser {
    state: PState,
    params: [i64; 16],
    num: usize,
    inter: [u8; 4],
    ninter: usize,
    osc: Vec<u8>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: PState::Ground,
            params: [0; 16],
            num: 0,
            inter: [0; 4],
            ninter: 0,
            osc: Vec::new(),
        }
    }

    fn reset_seq(&mut self) {
        self.num = 0;
        self.ninter = 0;
    }

    fn push_digit(&mut self, d: i64) {
        if self.num == 0 {
            self.num = 1;
            self.params[0] = 0;
        }
        let i = self.num - 1;
        if i < 16 {
            self.params[i] = (self.params[i] * 10 + d).min(65535);
        }
    }

    fn push_sep(&mut self) {
        if self.num == 0 {
            self.num = 1;
            self.params[0] = 0;
        }
        if self.num < 16 {
            self.num += 1;
            self.params[self.num - 1] = 0;
        }
    }

    fn push_inter(&mut self, b: u8) {
        if self.ninter < 4 {
            self.inter[self.ninter] = b;
            self.ninter += 1;
        }
    }

    pub fn advance<P: Perform>(&mut self, p: &mut P, b: u8) {
        match self.state {
            PState::Ground => match b {
                0x1B => self.state = PState::Escape,
                0x00..=0x1F => p.execute(b),
                0x7F => {}
                _ => p.print(b),
            },
            PState::Escape => {
                self.reset_seq();
                match b {
                    b'[' => self.state = PState::CsiEntry,
                    b']' => {
                        self.osc.clear();
                        self.state = PState::Osc;
                    }
                    0x20..=0x2F => {
                        self.push_inter(b);
                        self.state = PState::EscapeInter;
                    }
                    0x1B => {}
                    0x30..=0x7E => {
                        p.esc(b, &self.inter[..self.ninter]);
                        self.state = PState::Ground;
                    }
                    _ => self.state = PState::Ground,
                }
            }
            PState::EscapeInter => match b {
                0x20..=0x2F => self.push_inter(b),
                0x30..=0x7E => {
                    p.esc(b, &self.inter[..self.ninter]);
                    self.state = PState::Ground;
                }
                _ => self.state = PState::Ground,
            },
            PState::CsiEntry | PState::CsiParam => match b {
                0x30..=0x39 => {
                    self.push_digit((b - 0x30) as i64);
                    self.state = PState::CsiParam;
                }
                0x3B => {
                    self.push_sep();
                    self.state = PState::CsiParam;
                }
                0x3A => {}
                0x3C..=0x3F => self.push_inter(b),
                0x20..=0x2F => {
                    self.push_inter(b);
                    self.state = PState::CsiInter;
                }
                0x40..=0x7E => {
                    p.csi(b, &self.params[..self.num], &self.inter[..self.ninter]);
                    self.state = PState::Ground;
                }
                0x00..=0x1F => p.execute(b),
                _ => {}
            },
            PState::CsiInter => match b {
                0x20..=0x2F => self.push_inter(b),
                0x40..=0x7E => {
                    p.csi(b, &self.params[..self.num], &self.inter[..self.ninter]);
                    self.state = PState::Ground;
                }
                0x00..=0x1F => p.execute(b),
                _ => self.state = PState::Ground,
            },
            PState::Osc => match b {
                0x07 => {
                    p.osc(&self.osc);
                    self.state = PState::Ground;
                }
                0x1B => self.state = PState::OscEsc,
                _ => {
                    if self.osc.len() < 256 {
                        self.osc.push(b);
                    }
                }
            },
            PState::OscEsc => match b {
                b'\\' => {
                    p.osc(&self.osc);
                    self.state = PState::Ground;
                }
                _ => self.state = PState::Ground,
            },
        }
    }
}
