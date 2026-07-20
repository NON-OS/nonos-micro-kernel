// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for attribute-selector matching and the nth-child An+B
//! parser, driven through the real selector parser. These are the classic
//! self-consistent-but-wrong sites: the CSS attribute operators (^= $= *= ~= |=)
//! and the odd/even/negative forms of nth-child.

use crate::browser::css::parse::selectors::parse_selectors;
use crate::browser::css::selector::Pseudo;

// Parse "[...]" and test its single attribute operator against `have`.
fn attr_matches(sel: &str, have: &str) -> bool {
    let list = parse_selectors(sel);
    list[0].key.attrs[0].1.matches(have)
}

#[test]
fn attribute_operators_match_per_the_spec() {
    // ~=  whitespace-separated word
    assert!(attr_matches("[class~=btn]", "btn primary"));
    assert!(attr_matches("[class~=btn]", "btn"));
    assert!(!attr_matches("[class~=btn]", "btnx"), "not a substring match");
    assert!(!attr_matches("[class~=btn]", "primary"));

    // ^=  prefix, $=  suffix, *=  substring
    assert!(attr_matches("[href^=https]", "https://x"));
    assert!(!attr_matches("[href^=https]", "http://x"));
    assert!(attr_matches("[href$=.png]", "a.png"));
    assert!(!attr_matches("[href$=.png]", "a.jpg"));
    assert!(attr_matches("[title*=ell]", "hello"));
    assert!(!attr_matches("[title*=ell]", "world"));

    // |=  exact or value-then-hyphen (the language operator)
    assert!(attr_matches("[lang|=en]", "en"));
    assert!(attr_matches("[lang|=en]", "en-US"));
    assert!(!attr_matches("[lang|=en]", "english"), "must be en or en-*, not a prefix");
    assert!(!attr_matches("[lang|=en]", "fr"));

    // =  exact, case-sensitive
    assert!(attr_matches("[type=text]", "text"));
    assert!(!attr_matches("[type=text]", "Text"));

    // presence
    assert!(attr_matches("[data-x]", ""));
}

fn nth(sel: &str) -> (i32, i32) {
    let list = parse_selectors(sel);
    match list[0].key.pseudo.iter().find_map(|p| match p {
        Pseudo::NthChild(a, b) => Some((*a, *b)),
        _ => None,
    }) {
        Some(v) => v,
        None => panic!("no nth-child in {sel:?}"),
    }
}

#[test]
fn nth_child_parses_the_an_plus_b_forms() {
    assert_eq!(nth(":nth-child(2n+1)"), (2, 1), "the general form");
    assert_eq!(nth(":nth-child(odd)"), (2, 1), "odd == 2n+1");
    assert_eq!(nth(":nth-child(even)"), (2, 0), "even == 2n");
    assert_eq!(nth(":nth-child(3)"), (0, 3), "a bare integer is b with a=0");
    assert_eq!(nth(":nth-child(n)"), (1, 0), "n alone is 1n+0");
    assert_eq!(nth(":nth-child(2n-1)"), (2, -1), "a negative b");
    assert_eq!(nth(":nth-child(-n+3)"), (-1, 3), "a negative a, the first-three idiom");
}
