// NONOS Operating System (AGPL-3.0-or-later)
//! The engine's own module tree, compiled from the capsule's source. These
//! used to be hand-written mirrors listing every file, which drifted: the dom
//! mirror had been missing `measure` and `serialize` for a while. Pointing at
//! the real mod.rs keeps one list of what the engine is made of.

#[path = "../../../capsule_browser/src/browser/css/mod.rs"]
pub mod css;
#[path = "../../../capsule_browser/src/browser/dom/mod.rs"]
pub mod dom;
#[path = "../../../capsule_browser/src/browser/fonts/mod.rs"]
pub mod fonts;
#[path = "../../../capsule_browser/src/browser/html/mod.rs"]
pub mod html;
#[path = "../../../capsule_browser/src/browser/layout/mod.rs"]
pub mod layout;
#[path = "../../../capsule_browser/src/browser/url/mod.rs"]
pub mod url;

// js keeps its mirror: the interpreter modules under test are private to the
// capsule and nothing outside the engine has cause to reach them.
pub mod image;
pub mod js;
pub mod manifest;

#[path = "../../../capsule_browser/src/browser/http/chunked/mod.rs"]
pub mod chunked;
