// NONOS Operating System (AGPL-3.0-or-later)
// The engine source compiled here is held to the capsule crate own clippy
// gate, not this one. Vendoring it under -D warnings would make a lint in a
// file this crate does not own fail the proof run, so the three that reach
// across the boundary are allowed here and nowhere else.
#![allow(clippy::redundant_closure)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::too_many_arguments)]
//! Host proofs for the browser engine. Real engine source is included via
//! #[path] under a module tree mirroring the capsule's paths (real directories
//! so the relative includes resolve), so the files compile unchanged.
extern crate alloc;

pub mod browser;
pub mod grid_page;
pub mod render;

#[cfg(test)]
mod cascade_tests;
#[cfg(test)]
mod chunked_tests;
#[cfg(test)]
mod clone_tests;
#[cfg(test)]
mod color_tests;
#[cfg(test)]
mod dom_tests;
#[cfg(test)]
mod entity_tests;
#[cfg(test)]
mod grid_auto_tests;
#[cfg(test)]
mod grid_clip_tests;
#[cfg(test)]
mod grid_tests;
#[cfg(test)]
mod selector_tests;
#[cfg(test)]
mod table_tests;
#[cfg(test)]
mod url_tests;
