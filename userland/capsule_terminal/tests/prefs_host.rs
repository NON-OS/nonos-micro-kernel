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
//
// Host-side harness for the on-disk preferences record. Compiled with the
// host toolchain:
//   rustc --edition 2021 --test tests/prefs_host.rs -o /tmp/prefs_host && /tmp/prefs_host
//
// `codec.rs` reaches for `crate::term::dimensions`. In a `--test` binary the
// crate root is this file, so the module tree below is laid out to match the
// shipping one and the `crate::` path resolves against it. The font-scale
// bounds mirror `src/term/dimensions.rs`.

#[path = "../src/term"]
mod term {
    pub mod dimensions {
        pub const MIN_FONT_SCALE: u32 = 1;
        pub const MAX_FONT_SCALE: u32 = 6;
    }

    pub mod prefs {
        pub mod types;

        pub mod codec;
    }
}

use term::prefs::codec::{decode, encode, LEN, MAGIC, VERSION};
use term::prefs::types::Prefs;

fn same(a: &Prefs, b: &Prefs) -> bool {
    a.theme == b.theme
        && a.font_scale == b.font_scale
        && a.cursor == b.cursor
        && a.rails == b.rails
}

fn show(p: &Prefs) -> String {
    format!(
        "theme {} font_scale {} cursor {} rails {:#04b}",
        p.theme, p.font_scale, p.cursor, p.rails
    )
}

/// The record is written to a file that outlives the build, so its shape is a
/// compatibility contract, not an implementation detail.
#[test]
fn the_record_shape_is_the_documented_one() {
    assert_eq!(LEN, 12);
    assert_eq!(MAGIC, *b"NTP1");
    assert_eq!(VERSION, 1);
}

/// Every setting a reader can change must survive a save and a reboot; a
/// field dropped in the codec looks like the setting silently not sticking.
#[test]
fn every_field_round_trips() {
    let p = Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11 };
    let got = decode(&encode(&p));
    assert!(same(&got, &p), "expected {}, got {}", show(&p), show(&got));
}

/// A truncated file is what a crash mid-write leaves behind. It must read as
/// "no preferences yet", and above all must not index past the end.
#[test]
fn short_buffers_fall_back_to_defaults() {
    let full = encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11 });
    let d = Prefs::default();
    for n in 0..LEN {
        let got = decode(&full[..n]);
        assert!(
            same(&got, &d),
            "{} bytes decoded to {}, want defaults {}",
            n,
            show(&got),
            show(&d)
        );
    }
}

/// Some other file landing at the preferences path must not be reinterpreted
/// as settings.
#[test]
fn bad_magic_falls_back_to_defaults() {
    let mut b = encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11 });
    b[0] = b'X';
    let got = decode(&b);
    assert!(same(&got, &Prefs::default()), "got {}", show(&got));
}

/// A record written by a newer build has fields this one cannot interpret;
/// reading it as version 1 would apply garbage settings.
#[test]
fn an_unknown_version_falls_back_to_defaults() {
    let mut b = encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11 });
    b[4] = 7;
    b[5] = 0;
    let got = decode(&b);
    assert!(same(&got, &Prefs::default()), "got {}", show(&got));
}

/// The file is untrusted input. An out-of-range theme or cursor indexes a
/// table, and an out-of-range font scale sizes every glyph on screen.
#[test]
fn out_of_range_values_are_clamped() {
    let b = encode(&Prefs { theme: 9999, font_scale: 200, cursor: 99, rails: 0xFF });
    let got = decode(&b);
    assert!(got.theme < 4, "theme {} out of range", got.theme);
    assert!(
        got.font_scale >= 1 && got.font_scale <= 6,
        "font_scale {} out of range",
        got.font_scale
    );
    assert!(got.cursor < 4, "cursor {} out of range", got.cursor);
    assert!(got.rails <= 0b11, "rails {:#04b} out of range", got.rails);
}

/// These are the values `State::new` hardcoded before preferences existed, so
/// a first boot with no file on disk must look exactly like the old build.
#[test]
fn the_defaults_match_the_historical_hardcoded_values() {
    let d = Prefs::default();
    assert_eq!(d.font_scale, 2);
    assert_eq!(d.theme, 0);
    assert_eq!(d.cursor, 0);
}
