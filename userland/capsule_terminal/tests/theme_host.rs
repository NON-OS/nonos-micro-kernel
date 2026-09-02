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
// Host-side harness for the terminal colour profiles. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/theme_host.rs -o /tmp/theme_host && /tmp/theme_host
//
// `profiles.rs` reaches only for `super::types::Theme`, so the module tree
// below just has to mirror the shipping one; nothing needs stubbing. The luma
// weighting matches `src/paint/shade.rs::elevate`, which is what actually
// decides light-vs-dark at draw time.

#[path = "../src/term"]
mod term {
    pub mod theme {
        pub mod types;

        pub mod profiles;
    }
}

use term::theme::profiles::{by_index, by_name, ABYSS, COUNT, DARK, DIM, LIGHT};
use term::theme::types::Theme;

fn luma(c: u32) -> u32 {
    let (r, g, b) = ((c >> 16) & 0xFF, (c >> 8) & 0xFF, c & 0xFF);
    (r * 30 + g * 59 + b * 11) / 100
}

fn gap(a: u32, b: u32) -> u32 {
    let (x, y) = (luma(a), luma(b));
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn profiles() -> [(&'static str, Theme); 4] {
    [("DARK", DARK), ("DIM", DIM), ("LIGHT", LIGHT), ("ABYSS", ABYSS)]
}

/// The profiles replaced a set where the old `light` entry swapped only the
/// background and kept the near-white foreground constants, painting white on
/// white. Every profile must carry its own foreground far from its own ground.
#[test]
fn every_profile_separates_text_from_ground() {
    for (name, t) in profiles() {
        let d = gap(t.fg, t.bg);
        assert!(
            d > 90,
            "{}: fg {:#010X} luma {} vs bg {:#010X} luma {} — gap {}, want > 90",
            name,
            t.fg,
            luma(t.fg),
            t.bg,
            luma(t.bg),
            d
        );
    }
}

/// A status colour that vanishes into the ground is worse than no colour at
/// all: it silently drops the signal an error or a path was meant to carry.
#[test]
fn status_colours_are_legible_on_every_ground() {
    for (name, t) in profiles() {
        for (role, c) in [
            ("accent", t.accent),
            ("ok", t.ok),
            ("err", t.err),
            ("path", t.path),
            ("dim", t.dim),
            ("run", t.run),
        ] {
            let d = gap(c, t.bg);
            assert!(
                d > 40,
                "{}.{}: {:#010X} luma {} vs bg luma {} — gap {}, want > 40",
                name,
                role,
                c,
                luma(c),
                luma(t.bg),
                d
            );
        }
    }
}

/// `run` ("a command is running") and `dim` ("muted text") are distinct roles
/// that happen to share a value on the three dark grounds today — that
/// coincidence is legitimate and deliberately not asserted here. LIGHT is
/// where they were pulled apart on purpose, so only LIGHT is pinned.
#[test]
fn run_and_dim_stay_distinguishable_on_light() {
    assert_ne!(
        luma(LIGHT.run),
        luma(LIGHT.dim),
        "LIGHT run {:#010X} and dim {:#010X} both land on luma {}",
        LIGHT.run,
        LIGHT.dim,
        luma(LIGHT.run)
    );
}

/// The preferences record stores the theme as a bare `u16` read off disk, so
/// a corrupt or newer file can hand `by_index` any value at all. The lookup
/// must be total — saturating to DARK, never panicking.
#[test]
fn index_lookup_is_total_and_stable() {
    assert_eq!(COUNT, 4, "COUNT is {}", COUNT);
    assert_eq!(by_index(0).bg, DARK.bg);
    assert_eq!(by_index(3).bg, ABYSS.bg);
    assert_eq!(
        by_index(99).bg,
        DARK.bg,
        "out-of-range index gave {:#010X}, want DARK {:#010X}",
        by_index(99).bg,
        DARK.bg
    );
}

/// Nine profiles collapsed to four. The retired names stay resolvable so an
/// existing `theme blackarch` in muscle memory or a script keeps working
/// instead of erroring out.
#[test]
fn names_resolve_including_retired_ones() {
    for (n, want) in [
        (&b"dark"[..], 0u16),
        (&b"dim"[..], 1),
        (&b"light"[..], 2),
        (&b"abyss"[..], 3),
        (&b"blackarch"[..], 0),
        (&b"black"[..], 0),
        (&b"blue"[..], 0),
        (&b"matrix"[..], 0),
        (&b"glass"[..], 1),
        (&b"smoke"[..], 1),
        (&b"clear"[..], 1),
    ] {
        let got = by_name(n);
        assert_eq!(
            got,
            Some(want),
            "by_name({:?}) = {:?}, want Some({})",
            core::str::from_utf8(n).unwrap(),
            got,
            want
        );
    }
    assert_eq!(by_name(b"nope"), None);
}

/// Guards against a future edit quietly turning "light" into a mid-grey, or
/// letting a dark ground drift up until the whole surface washes out.
#[test]
fn light_is_light_and_the_rest_are_dark() {
    assert!(
        luma(LIGHT.bg) > 200,
        "LIGHT.bg {:#010X} luma {}, want > 200",
        LIGHT.bg,
        luma(LIGHT.bg)
    );
    for (name, t) in [("DARK", DARK), ("DIM", DIM), ("ABYSS", ABYSS)] {
        assert!(
            luma(t.bg) < 60,
            "{}.bg {:#010X} luma {}, want < 60",
            name,
            t.bg,
            luma(t.bg)
        );
    }
}

/// The default look must not shift underfoot. Only the background is pinned:
/// DARK's accents deliberately DID move in the refactor (accent 0x3FD0C9 ->
/// 0x5AE6D0, with path/ok/err following). Do not "fix" this test by adding
/// the old accent values back — they are intentionally gone.
#[test]
fn darks_background_is_unchanged_from_the_pre_refactor_constant() {
    assert_eq!(
        DARK.bg, 0xFF07_090B,
        "DARK.bg is {:#010X}, want 0xFF07090B",
        DARK.bg
    );
}
