// NONOS Operating System (AGPL-3.0-or-later)
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
