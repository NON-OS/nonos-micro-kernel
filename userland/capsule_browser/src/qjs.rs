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

//! Startup proof of the whole QuickJS page path: a page whose `<script>`
//! registers a click handler with closure state is run by the executor, then
//! dispatched events fire the handler and grow the DOM. The engine returned by
//! the executor holds the listeners between run and dispatch, as a live page must.

use alloc::format;
use alloc::string::ToString;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;
use crate::browser::qjs_run::run_scripts;

const PAGE_JS: &str = "var app=document.getElementById('app');\
     var btn=document.createElement('button'); btn.id='btn'; btn.textContent='Add';\
     var count=0;\
     btn.addEventListener('click', function(e){ count++; var d=document.createElement('div');\
       d.className='added'; d.textContent='clicked '+count; app.appendChild(d); });\
     app.appendChild(btn);";

pub fn selftest() {
    let mut dom = Dom::new();
    if let Some(app) = dom.create(NodeKind::Element, "div".to_string()) {
        dom.set_attr(app, "id", "app".to_string());
        if let Some(script) = dom.create(NodeKind::Element, "script".to_string()) {
            if let Some(text) = dom.create(NodeKind::Text, PAGE_JS.to_string()) {
                dom.attach(script, text);
            }
            dom.attach(app, script);
        }
    }
    let Some(engine) = run_scripts(&mut dom) else {
        log(b"[QJS] executor init failed\n");
        return;
    };
    let before = dom.nodes.len();
    let btn = find_id(&dom, "btn");
    let mut fired = 0;
    if btn >= 0 {
        for _ in 0..3 {
            fired += engine.dispatch_event(btn, "click");
        }
    }
    log(format!("[QJS] page + events: {before} nodes -> {} after 3 clicks ({fired} handlers)\n", dom.nodes.len()).as_bytes());
}

fn find_id(dom: &Dom, id: &str) -> i32 {
    for (i, n) in dom.nodes.iter().enumerate() {
        if n.attrs.iter().any(|(k, v)| k == "id" && v == id) {
            return i as i32;
        }
    }
    -1
}

fn log(b: &[u8]) {
    let _ = nonos_libc::mk_debug(b.as_ptr(), b.len());
}
