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

//! The QuickJS page-script executor. Runs every `<script>` in a page DOM through
//! QuickJS with that DOM installed, so the scripts build and mutate the real
//! tree. The caller lays the mutated tree out afterwards, which is the reflow.

use core::ffi::c_void;

use nonos_qjs::Engine;

use super::dom::Dom;
use super::js::collect_scripts;

/// Run the DOM's scripts through QuickJS against that DOM and return the engine,
/// which keeps the page's listeners and closure state alive so later UI events
/// can be dispatched into it. The DOM pointer lives in the engine's context, so
/// `dom` must outlive the returned engine.
pub fn run_scripts(dom: &mut Dom) -> Option<Engine> {
    let scripts = collect_scripts(dom);
    let engine = Engine::new()?;
    engine.install_dom(dom as *mut Dom as *mut c_void);
    for s in &scripts {
        let _ = engine.eval(s);
    }
    Some(engine)
}
