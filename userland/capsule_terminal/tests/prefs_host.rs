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

    pub mod theme {
        pub mod profiles {
            pub const COUNT: u16 = 4;
        }
    }

    pub mod prefs {
        pub mod types;

        pub mod projects;

        pub mod codec;
    }
}

use term::prefs::codec::{decode, encode, HEAD, LEN, MAGIC, VERSION};
use term::prefs::types::{Prefs, MAX_PROJECTS, PATH_CAP};

fn same(a: &Prefs, b: &Prefs) -> bool {
    a.theme == b.theme
        && a.font_scale == b.font_scale
        && a.cursor == b.cursor
        && a.rails == b.rails
        && a.project_slice() == b.project_slice()
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
    assert_eq!(HEAD, 12);
    assert_eq!(LEN, HEAD + 1 + MAX_PROJECTS * (1 + PATH_CAP));
    assert_eq!(MAGIC, *b"NTP1");
    assert_eq!(VERSION, 2);
}

/// Every setting a reader can change must survive a save and a reboot; a
/// field dropped in the codec looks like the setting silently not sticking.
#[test]
fn every_field_round_trips() {
    let p = Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() };
    let got = decode(&encode(&p));
    assert!(same(&got, &p), "expected {}, got {}", show(&p), show(&got));
}

/// A truncated file is what a crash mid-write leaves behind. It must read as
/// "no preferences yet", and above all must not index past the end.
#[test]
fn short_buffers_fall_back_to_defaults() {
    let full = encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() });
    let d = Prefs::default();
    for n in 0..HEAD {
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
    let mut b = encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() });
    b[0] = b'X';
    let got = decode(&b);
    assert!(same(&got, &Prefs::default()), "got {}", show(&got));
}

/// A record written by a newer build has fields this one cannot interpret;
/// reading it as version 1 would apply garbage settings.
#[test]
fn an_unknown_version_falls_back_to_defaults() {
    let mut b = encode(&Prefs { theme: 3, font_scale: 5, cursor: 2, rails: 0b11, ..Prefs::default() });
    b[4] = 7;
    b[5] = 0;
    let got = decode(&b);
    assert!(same(&got, &Prefs::default()), "got {}", show(&got));
}

/// The file is untrusted input. An out-of-range theme or cursor indexes a
/// table, and an out-of-range font scale sizes every glyph on screen.
#[test]
fn out_of_range_values_are_clamped() {
    let b = encode(&Prefs { theme: 9999, font_scale: 200, cursor: 99, rails: 0xFF, ..Prefs::default() });
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

/// Pinned project paths are settings like any other: dropping them in the codec
/// looks like the rail forgetting what the user saved.
#[test]
fn projects_round_trip() {
    let mut p = Prefs::default();
    assert!(p.push_project(b"/home/user/src"));
    assert!(p.push_project(b"/etc"));
    let got = decode(&encode(&p));
    assert!(same(&got, &p), "projects lost: {} slots back", got.project_count);
    assert_eq!(got.project_slice()[0].as_str(), "/home/user/src");
    assert_eq!(got.project_slice()[1].as_str(), "/etc");
}

/// A record whose head is intact but whose blob was cut short must still yield
/// the settings it does carry, with no projects, rather than defaults or a panic.
#[test]
fn a_truncated_project_blob_keeps_the_head_fields() {
    let mut p = Prefs::default();
    p.theme = 3;
    p.font_scale = 5;
    assert!(p.push_project(b"/home/user/src"));
    let full = encode(&p);
    for n in HEAD..LEN {
        let got = decode(&full[..n]);
        assert_eq!(got.theme, 3, "{n} bytes lost the theme");
        assert_eq!(got.font_scale, 5, "{n} bytes lost the font scale");
        assert_eq!(got.project_count, 0, "{n} bytes kept a partial project");
    }
}

/// The slot table is fixed, and the file is untrusted: a count or a length past
/// the table must be clamped rather than indexed.
#[test]
fn a_hostile_project_blob_is_clamped() {
    let mut b = encode(&Prefs::default());
    b[HEAD] = 0xFF;
    for slot in b[HEAD + 1..].iter_mut() {
        *slot = 0xFF;
    }
    let got = decode(&b);
    assert!(got.project_count as usize <= MAX_PROJECTS, "count {}", got.project_count);
    for pr in got.project_slice() {
        assert!(pr.as_bytes().len() <= PATH_CAP, "slot overran the cap");
    }
}

/// The table is full at `MAX_PROJECTS`, and pinning the same path twice would
/// spend a slot on a row the user already has.
#[test]
fn pinning_is_bounded_and_deduplicated() {
    let mut p = Prefs::default();
    assert!(p.push_project(b"/a"));
    assert!(!p.push_project(b"/a"));
    assert!(!p.push_project(b""));
    for i in 1..MAX_PROJECTS {
        assert!(p.push_project(format!("/p{i}").as_bytes()), "slot {i} refused");
    }
    assert_eq!(p.project_count as usize, MAX_PROJECTS);
    assert!(!p.push_project(b"/overflow"));
}

/// A record written before the projects table existed is still a valid record:
/// its head is byte-identical to the current one, so the user's theme and font
/// scale must survive the upgrade rather than silently resetting to defaults.
#[test]
fn a_v1_record_keeps_its_head_fields() {
    let mut p = Prefs::default();
    p.theme = 2;
    p.font_scale = 3;
    let mut b = encode(&p);
    b[4..6].copy_from_slice(&1u16.to_le_bytes());
    let got = decode(&b[..HEAD]);
    assert_eq!(got.theme, 2, "v1 theme lost");
    assert_eq!(got.font_scale, 3, "v1 font scale lost");
    assert_eq!(got.project_count, 0, "v1 record cannot carry projects");
}

/// A version from the future is not decodable and must fall back cleanly.
#[test]
fn a_future_version_falls_back_to_defaults() {
    let mut b = encode(&Prefs::default());
    b[4..6].copy_from_slice(&(VERSION + 1).to_le_bytes());
    assert_eq!(decode(&b).theme, Prefs::default().theme);
}
