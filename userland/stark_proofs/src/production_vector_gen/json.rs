// NONOS Operating System (AGPL-3.0-or-later)
//! Field-element JSON formatting shared by the artifact emitters: Fp as a
//! decimal string, Fp2 as [c0, c1], digests as 0x-hex.

use crate::crypto::stark::field::{Fp, Fp2};
use alloc::string::String;
use alloc::vec::Vec;

pub(super) fn fp(v: Fp) -> String {
    alloc::format!("\"{}\"", v.value())
}

pub(super) fn fp2(v: Fp2) -> String {
    alloc::format!("[\"{}\",\"{}\"]", v.c0.value(), v.c1.value())
}

pub(super) fn fps(vs: &[Fp]) -> String {
    list(vs.iter().map(|v| fp(*v)))
}

pub(super) fn fp2s(vs: &[Fp2]) -> String {
    list(vs.iter().map(|v| fp2(*v)))
}

pub(super) fn usizes(vs: &[usize]) -> String {
    list(vs.iter().map(|v| alloc::format!("{}", v)))
}

pub(super) fn bools(vs: &[bool]) -> String {
    list(vs.iter().map(|v| String::from(if *v { "true" } else { "false" })))
}

pub(super) fn hex32(d: &[u8; 32]) -> String {
    let mut s = String::from("0x");
    for b in d {
        s.push_str(&alloc::format!("{:02x}", b));
    }
    s
}

fn list<I: Iterator<Item = String>>(items: I) -> String {
    let parts: Vec<String> = items.collect();
    let mut s = String::from("[");
    s.push_str(&parts.join(","));
    s.push(']');
    s
}
