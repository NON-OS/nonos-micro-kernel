// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for CSS specificity, the heart of the cascade: which
//! rule wins when two match. CSS 2.1 6.4.3 compares (ids, classes+attrs, tags)
//! as a tuple, so a higher level always outranks any count at a lower one. The
//! engine packs that tuple into one comparable u32 (ten bits per level); these
//! decode it to check the counts and assert the ordering the cascade relies on.

use crate::browser::css::parse::selectors::parse_selectors;
use crate::browser::css::specificity::specificity;

fn spec(sel: &str) -> u32 {
    let list = parse_selectors(sel);
    assert_eq!(list.len(), 1, "expected a single selector for {sel:?}");
    specificity(&list[0])
}

// Decode the packed value back into (ids, classes+attrs, tags).
fn counts(sel: &str) -> (u32, u32, u32) {
    let s = spec(sel);
    ((s >> 20) & 0x3FF, (s >> 10) & 0x3FF, s & 0x3FF)
}

#[test]
fn each_simple_selector_counts_at_its_own_level() {
    assert_eq!(counts("div"), (0, 0, 1), "a tag");
    assert_eq!(counts(".card"), (0, 1, 0), "a class");
    assert_eq!(counts("#main"), (1, 0, 0), "an id");
    assert_eq!(counts("[data-x]"), (0, 1, 0), "an attribute counts as a class");
}

#[test]
fn compound_and_chain_selectors_accumulate_per_level() {
    assert_eq!(counts("div.card#main"), (1, 1, 1));
    assert_eq!(counts("a.b.c"), (0, 2, 1), "tag + two classes");
    assert_eq!(counts("input[type=text].field"), (0, 2, 1), "tag + attr + class");
    assert_eq!(counts("div p"), (0, 0, 2), "two tags down a descendant chain");
    assert_eq!(counts("#nav > .item a"), (1, 1, 1), "id + class + tag across the chain");
}

#[test]
fn one_id_outranks_any_number_of_classes() {
    // The tuple guarantee, and the exact bug a flattened id*100+class*10 scheme
    // gets wrong: ten classes must NOT reach a single id.
    assert!(spec("#x") > spec(".a.b.c.d.e.f.g.h.i.j"), "one id beats ten classes");
    assert!(spec("#x") > spec(".a.b.c.d.e"), "and beats five");
}

#[test]
fn the_levels_are_strictly_ordered() {
    assert!(spec("#main") > spec(".card"), "id beats class");
    assert!(spec(".card") > spec("div"), "class beats tag");
    assert!(spec("#a #b") > spec("#a"), "two ids beat one");
    assert!(spec(".a.b") > spec(".a"), "two classes beat one at the same level");
    assert!(spec("div.card") > spec("div"), "adding a class raises specificity");
}

#[test]
fn a_selector_list_parses_into_independent_selectors() {
    let list = parse_selectors("div, .card, #main");
    assert_eq!(list.len(), 3, "three comma-separated selectors");
    assert!(specificity(&list[2]) > specificity(&list[1]));
    assert!(specificity(&list[1]) > specificity(&list[0]));
}
