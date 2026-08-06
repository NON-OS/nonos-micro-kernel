// NONOS Operating System (AGPL-3.0-or-later)
//! The real DOM tree builder, included unchanged so proofs run over the
//! parser the capsule actually ships.

#[path = "../../../../capsule_browser/src/browser/dom/attach.rs"]
mod attach;
#[path = "../../../../capsule_browser/src/browser/dom/attrs.rs"]
mod attrs;
#[path = "../../../../capsule_browser/src/browser/dom/auto_close.rs"]
mod auto_close;
#[path = "../../../../capsule_browser/src/browser/dom/close_tag.rs"]
mod close_tag;
#[path = "../../../../capsule_browser/src/browser/dom/comment.rs"]
mod comment;
#[path = "../../../../capsule_browser/src/browser/dom/consume.rs"]
mod consume;
#[path = "../../../../capsule_browser/src/browser/dom/create.rs"]
mod create;
#[path = "../../../../capsule_browser/src/browser/dom/detach.rs"]
mod detach;
#[path = "../../../../capsule_browser/src/browser/dom/flush_text.rs"]
mod flush_text;
#[path = "../../../../capsule_browser/src/browser/dom/insert_before.rs"]
mod insert_before;
#[path = "../../../../capsule_browser/src/browser/dom/limits.rs"]
mod limits;
#[path = "../../../../capsule_browser/src/browser/dom/node.rs"]
pub mod node;
#[path = "../../../../capsule_browser/src/browser/dom/parse.rs"]
mod parse;
#[path = "../../../../capsule_browser/src/browser/dom/push.rs"]
mod push;
#[path = "../../../../capsule_browser/src/browser/dom/raw_text.rs"]
mod raw_text;
#[path = "../../../../capsule_browser/src/browser/dom/remove_attr.rs"]
mod remove_attr;
#[path = "../../../../capsule_browser/src/browser/dom/set_attr.rs"]
mod set_attr;
#[path = "../../../../capsule_browser/src/browser/dom/tree.rs"]
pub mod tree;
#[path = "../../../../capsule_browser/src/browser/dom/void.rs"]
mod void;

pub use parse::parse;
pub use tree::Dom;
