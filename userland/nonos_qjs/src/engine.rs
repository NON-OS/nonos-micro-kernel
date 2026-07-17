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

//! Safe Rust surface over the QuickJS runtime. A single owned runtime and
//! context per Engine; values never cross the boundary, only strings.

use alloc::string::String;
use core::ffi::c_void;

use crate::alloc_stubs::free;

extern "C" {
    fn njs_new_runtime() -> *mut c_void;
    fn njs_new_context(rt: *mut c_void) -> *mut c_void;
    fn njs_free_context(ctx: *mut c_void);
    fn njs_free_runtime(rt: *mut c_void);
    fn njs_eval_to_string(ctx: *mut c_void, code: *const u8, len: usize) -> *mut u8;
    fn njs_install_dom(ctx: *mut c_void, host: *mut c_void);
    fn njs_dispatch_event(ctx: *mut c_void, node: i32, ty: *const u8) -> i32;
}

pub struct Engine {
    rt: *mut c_void,
    ctx: *mut c_void,
}

impl Engine {
    /// Create a fresh runtime and context. Returns None if the engine could not
    /// allocate.
    pub fn new() -> Option<Engine> {
        unsafe {
            let rt = njs_new_runtime();
            if rt.is_null() {
                return None;
            }
            let ctx = njs_new_context(rt);
            if ctx.is_null() {
                njs_free_runtime(rt);
                return None;
            }
            Some(Engine { rt, ctx })
        }
    }

    /// Install the `document` global, binding DOM methods to `host`. The host
    /// pointer is handed to every njs_dom_* callback as the node tree to mutate;
    /// it must outlive every eval that touches the DOM.
    pub fn install_dom(&self, host: *mut c_void) {
        unsafe { njs_install_dom(self.ctx, host) }
    }

    /// Dispatch a UI event of `ty` to the listeners registered on `node`,
    /// returning how many fired. Used when a real pointer or key event lands on
    /// a laid-out node; the engine must be the one that ran the page scripts.
    pub fn dispatch_event(&self, node: i32, ty: &str) -> i32 {
        let mut buf = [0u8; 32];
        let n = ty.len().min(31);
        buf[..n].copy_from_slice(&ty.as_bytes()[..n]);
        unsafe { njs_dispatch_event(self.ctx, node, buf.as_ptr()) }
    }

    /// Evaluate a script and return its result coerced to a string, or the
    /// exception message. Pending jobs are drained before the result is read.
    pub fn eval(&self, code: &str) -> String {
        unsafe {
            let p = njs_eval_to_string(self.ctx, code.as_ptr(), code.len());
            if p.is_null() {
                return String::new();
            }
            let mut n = 0;
            while *p.add(n) != 0 {
                n += 1;
            }
            let out = String::from_utf8_lossy(core::slice::from_raw_parts(p, n)).into_owned();
            free(p);
            out
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            njs_free_context(self.ctx);
            njs_free_runtime(self.rt);
        }
    }
}
