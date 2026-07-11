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

//! Proofs for the shared keyboard layout tables. Every assertion is a
//! spot check against the authoritative national layout: if a table entry
//! drifts, the exact key that broke names itself in the failure.

use nonos_keymap::{resolve, Layout};

fn r(base: u8, shift: bool, layout: Layout) -> u32 {
    resolve(base as u32, shift, false, layout)
}

fn c(ch: char) -> u32 {
    ch as u32
}

#[test]
fn us_shift_symbols() {
    assert_eq!(r(b'-', true, Layout::Us), c('_'));
    assert_eq!(r(b'2', true, Layout::Us), c('@'));
    assert_eq!(r(b'/', true, Layout::Us), c('?'));
    assert_eq!(r(b';', true, Layout::Us), c(':'));
    assert_eq!(r(b'-', false, Layout::Us), c('-'));
}

#[test]
fn letters_case_from_shift_and_caps() {
    assert_eq!(resolve(b'a' as u32, false, false, Layout::Us), c('a'));
    assert_eq!(resolve(b'a' as u32, true, false, Layout::Us), c('A'));
    assert_eq!(resolve(b'a' as u32, false, true, Layout::Us), c('A'));
    // Shift under caps-lock gives lowercase again, like a real keyboard.
    assert_eq!(resolve(b'a' as u32, true, true, Layout::Us), c('a'));
}

#[test]
fn uk_swaps_quote_and_at_and_puts_pound_on_three() {
    assert_eq!(r(b'2', true, Layout::Uk), c('"'));
    assert_eq!(r(b'\'', true, Layout::Uk), c('@'));
    assert_eq!(r(b'3', true, Layout::Uk), 0x00A3);
    assert_eq!(r(b'\\', false, Layout::Uk), c('#'));
    // Everything the UK table does not name falls back to US.
    assert_eq!(r(b'-', true, Layout::Uk), c('_'));
}

#[test]
fn de_swaps_yz_and_places_umlauts() {
    assert_eq!(r(b'y', false, Layout::De), c('z'));
    assert_eq!(r(b'z', false, Layout::De), c('y'));
    assert_eq!(resolve(b'z' as u32, true, false, Layout::De), c('Y'));
    assert_eq!(r(b'[', false, Layout::De), 0x00FC); // u-umlaut
    assert_eq!(r(b'[', true, Layout::De), 0x00DC);
    assert_eq!(r(b';', false, Layout::De), 0x00F6); // o-umlaut
    assert_eq!(r(b'-', false, Layout::De), 0x00DF); // eszett
    assert_eq!(r(b'7', true, Layout::De), c('/'));
    assert_eq!(r(b'/', true, Layout::De), c('_'));
}

#[test]
fn fr_azerty_letters_and_digit_row() {
    assert_eq!(r(b'q', false, Layout::Fr), c('a'));
    assert_eq!(r(b'a', false, Layout::Fr), c('q'));
    assert_eq!(r(b'w', false, Layout::Fr), c('z'));
    assert_eq!(r(b'z', false, Layout::Fr), c('w'));
    // M sits on the US semicolon key; the US m key produces comma.
    assert_eq!(r(b';', false, Layout::Fr), c('m'));
    assert_eq!(resolve(b';' as u32, true, false, Layout::Fr), c('M'));
    assert_eq!(r(b'm', false, Layout::Fr), c(','));
    assert_eq!(r(b'm', true, Layout::Fr), c('?'));
    // Digits need shift; the unshifted row carries accented letters.
    assert_eq!(r(b'2', false, Layout::Fr), 0x00E9); // e-acute
    assert_eq!(r(b'2', true, Layout::Fr), c('2'));
    assert_eq!(r(b'0', false, Layout::Fr), 0x00E0); // a-grave
    assert_eq!(r(b'8', false, Layout::Fr), c('_'));
}

#[test]
fn es_places_ntilde_and_inverted_punctuation() {
    assert_eq!(r(b';', false, Layout::Es), 0x00F1); // ntilde
    assert_eq!(r(b';', true, Layout::Es), 0x00D1);
    assert_eq!(r(b'=', false, Layout::Es), 0x00A1); // inverted exclamation
    assert_eq!(r(b'=', true, Layout::Es), 0x00BF); // inverted question
    assert_eq!(r(b'/', true, Layout::Es), c('_'));
}

#[test]
fn it_places_accented_vowels() {
    assert_eq!(r(b'[', false, Layout::It), 0x00E8); // e-grave
    assert_eq!(r(b'[', true, Layout::It), 0x00E9); // e-acute
    assert_eq!(r(b';', false, Layout::It), 0x00F2); // o-grave
    assert_eq!(r(b'/', true, Layout::It), c('_'));
    assert_eq!(r(b'`', false, Layout::It), c('\\'));
}

#[test]
fn non_printable_codes_pass_through() {
    // Navigation and modifier keycodes live outside the ASCII base range.
    assert_eq!(resolve(0x1203, true, true, Layout::Fr), 0x1203);
    assert_eq!(resolve(0x0D, true, false, Layout::De), 0x0D);
}

#[test]
fn layout_cycle_visits_all_and_wraps() {
    let mut l = Layout::Us;
    let mut seen = [false; Layout::COUNT as usize];
    for _ in 0..Layout::COUNT {
        seen[l.index() as usize] = true;
        l = l.next();
    }
    assert_eq!(l, Layout::Us);
    assert!(seen.iter().all(|&s| s));
}
