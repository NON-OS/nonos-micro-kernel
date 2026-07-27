// NONOS Operating System (AGPL-3.0-or-later)
//! The witness-mode recursion assembly: nine regions and their grand-product
//! bindings, one region or binding family per file. `build` assembles the
//! wired AIR and witness from the real inner join-split proof; `tamper` names
//! the targeted forgeries the reject gate must catch.

pub(crate) mod auth;
pub(crate) mod build;
pub(crate) mod compose;
pub(crate) mod deep;
pub(crate) mod fri;
pub(crate) mod groups;
pub(crate) mod inner;
pub(crate) mod layout;
pub(crate) mod periodic;
pub(crate) mod points;
pub(crate) mod sponge;
pub(crate) mod tamper;
pub(crate) mod transcript;

pub(crate) use build::assemble;
pub(crate) use tamper::Tamper;
