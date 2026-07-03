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
