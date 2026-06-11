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

use core::arch::asm;

use super::fault::{parse_par, TranslationFault};

pub fn translate_stage1_read(virt: u64) -> Result<u64, TranslationFault> {
    translate(virt, "s1e1r")
}

pub fn translate_stage1_write(virt: u64) -> Result<u64, TranslationFault> {
    translate(virt, "s1e1w")
}

pub fn translate_user_read(virt: u64) -> Result<u64, TranslationFault> {
    translate(virt, "s1e0r")
}

pub fn translate_user_write(virt: u64) -> Result<u64, TranslationFault> {
    translate(virt, "s1e0w")
}

fn translate(virt: u64, mode: &str) -> Result<u64, TranslationFault> {
    let par: u64;
    unsafe {
        match mode {
            "s1e1w" => {
                asm!("at s1e1w, {0}", "isb", "mrs {1}, par_el1", in(reg) virt, out(reg) par, options(nostack))
            }
            "s1e0r" => {
                asm!("at s1e0r, {0}", "isb", "mrs {1}, par_el1", in(reg) virt, out(reg) par, options(nostack))
            }
            "s1e0w" => {
                asm!("at s1e0w, {0}", "isb", "mrs {1}, par_el1", in(reg) virt, out(reg) par, options(nostack))
            }
            _ => {
                asm!("at s1e1r, {0}", "isb", "mrs {1}, par_el1", in(reg) virt, out(reg) par, options(nostack))
            }
        }
    }
    parse_par(par, virt)
}
