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

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::obj::obj;

pub fn install(env: &Env) {
    env.define("console", obj(&[("log", Value::Native("console.log"))]));
    env.define(
        "document",
        obj(&[
            ("getElementById", Value::Native("document.getElementById")),
            ("querySelector", Value::Native("document.querySelector")),
            ("querySelectorAll", Value::Native("document.querySelectorAll")),
            ("createElement", Value::Native("document.createElement")),
            ("createTextNode", Value::Native("document.createTextNode")),
            ("createDocumentFragment", Value::Native("document.createDocumentFragment")),
            // Marks this object as the document, so reading body, head or
            // documentElement goes to the tree rather than to this map.
            ("__document__", Value::Bool(true)),
        ]),
    );
    env.define(
        "Math",
        obj(&[
            ("floor", Value::Native("Math.floor")),
            ("round", Value::Native("Math.round")),
            ("abs", Value::Native("Math.abs")),
            ("max", Value::Native("Math.max")),
            ("min", Value::Native("Math.min")),
        ]),
    );
    env.define(
        "JSON",
        obj(&[
            ("parse", Value::Native("JSON.parse")),
            ("stringify", Value::Native("JSON.stringify")),
        ]),
    );
    env.define(
        "Object",
        obj(&[
            ("keys", Value::Native("Object.keys")),
            ("values", Value::Native("Object.values")),
            ("entries", Value::Native("Object.entries")),
            ("assign", Value::Native("Object.assign")),
        ]),
    );
    env.define(
        "Promise",
        obj(&[
            ("resolve", Value::Native("Promise.resolve")),
            ("reject", Value::Native("Promise.reject")),
            ("__native_ctor__", Value::Str(alloc::rc::Rc::new("Promise".into()))),
        ]),
    );
    env.define(
        "RegExp",
        obj(&[("__native_ctor__", Value::Str(alloc::rc::Rc::new("RegExp".into())))]),
    );
    for name in ["Error", "TypeError", "RangeError", "SyntaxError", "ReferenceError", "Map", "Set"]
    {
        env.define(name, obj(&[("__native_ctor__", Value::Str(alloc::rc::Rc::new(name.into())))]));
    }
    env.define("window", obj(&[]));
    env.define("fetch", Value::Native("fetch"));
    env.define("setTimeout", Value::Native("setTimeout"));
    env.define("setInterval", Value::Native("setInterval"));
    env.define("parseInt", Value::Native("parseInt"));
    env.define("parseFloat", Value::Native("parseFloat"));
    env.define("Number", Value::Native("Number"));
    env.define("String", Value::Native("String"));
    env.define("Boolean", Value::Native("Boolean"));
    env.define("isNaN", Value::Native("isNaN"));
}
