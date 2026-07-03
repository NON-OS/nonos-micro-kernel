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

use alloc::string::String;

use crate::browser::http::response::{ContentKind, Response};
use crate::browser::state::{State, View};

use super::{record_history, render_response};

// Internal pages served without the network, rendered through the same
// HTML pipeline as a fetched document. about:engine shows the layout and
// script engine working; any other about: name gets the same page.
const ENGINE_HTML: &[u8] = br#"<html><head><style>
body{background:#10151a;color:#dbe6ee}
.bar{display:flex;justify-content:space-between;align-items:center;
 background:#1b2733;border:1px solid #46a6b2;border-radius:10px;
 padding:10px 14px;margin:10px 0}
.bar b{color:#8cdfea}
.tag{background:#46a6b2;color:#0c1116;border-radius:8px;padding:4px 10px}
.grid{display:grid;grid-template-columns:repeat(3,1fr);gap:10px;margin:12px 0}
.card{background:#18222c;border:1px solid #33424f;border-radius:8px;
 padding:10px;min-height:40px}
.card h3{font-size:16px;margin:0 0 6px 0;color:#9ad1ff}
.hero{position:relative;background:#141c24;border:1px solid #33424f;
 border-radius:12px;padding:16px;margin:12px 0}
.badge{position:absolute;top:10px;right:12px;background:#d08a2b;
 color:#141c24;border-radius:8px;padding:3px 9px}
.center{text-align:center;color:#7fa6b8}
button{background:#2b3844;border:1px solid #46a6b2;border-radius:8px}
#out{color:#ffd479}
#clock{color:#8cdfea}
</style></head><body>
<div class="bar"><b>NONOS box engine</b><span class="tag">live</span></div>
<div class="hero"><span class="badge">absolute</span>
<h2>Rendered by the new layout engine</h2>
<p>Flex, grid, borders, radius, margins, padding, real font metrics,
positioned boxes and inline wrapping are all active on this page.</p></div>
<div class="grid">
<div class="card"><h3>Flex</h3><p>space-between bar above</p></div>
<div class="card"><h3>Grid</h3><p>three fr tracks with gap</p></div>
<div class="card"><h3>Script</h3><p id="out">no clicks yet</p></div>
</div>
<form><input name="q" placeholder="type here and press enter"></form>
<button id="go">click me</button>
<p class="center">ticks: <span id="clock">0</span></p>
<script>
var n = 0;
var names = ["flex","grid","radius"].map(function(s){return s.toUpperCase();});
document.getElementById("go").addEventListener("click", function(){
  n = n + 1;
  var word = n === 1 ? "click" : "clicks";
  document.getElementById("out").textContent = `${n} ${word} (${names.join("+")})`;
});
var t = 0;
setInterval(function(){
  t = t + 1;
  document.getElementById("clock").textContent = String(t);
}, 1000);
</script></body></html>"#;

// Serve about: addresses locally. Returns true when the target was handled.
pub fn about_page(state: &mut State, target: &str) -> bool {
    let Some(_name) = target.strip_prefix("about:") else {
        return false;
    };
    let resp = Response {
        status: 200,
        body: ENGINE_HTML.to_vec(),
        location: None,
        content_kind: ContentKind::Html,
    };
    let (rendered, count) = render_response::render_response(&resp);
    state.scroll = 0;
    state.focus = None;
    state.status = alloc::format!("{} fl={}", target, count);
    match rendered {
        render_response::Rendered::Boxes(b, dom, world) => {
            state.box_doc = Some(b);
            state.page_dom = Some(dom);
            state.world = Some(world);
            state.document = None;
        }
        render_response::Rendered::Lines(d) => {
            state.document = Some(d);
            state.box_doc = None;
            state.page_dom = None;
            state.world = None;
        }
        render_response::Rendered::Nothing => {
            state.document = None;
            state.box_doc = None;
            state.page_dom = None;
            state.world = None;
        }
    }
    state.address = String::from(target);
    let suppress = core::mem::take(&mut state.suppress_history_push);
    record_history::record_history(state, suppress);
    state.view = View::Page;
    true
}
