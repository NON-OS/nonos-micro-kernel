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

use alloc::vec::Vec;

use super::parse::parse;
use super::rule::Rule;

const UA: &str = concat!(
    "head{display:none}script{display:none}style{display:none}title{display:none}",
    "a{color:#4c9aff;text-decoration:underline}b{font-weight:bold}strong{font-weight:bold}",
    "body{display:block;margin:8px}",
    "div,section,article,main,header,footer,nav,aside,figure,figcaption,form,button,",
    "thead,tbody,tfoot,li,dl,dt,dd{display:block}",
    // Tables lay each row as a flex line and each cell as an equal share, so
    // rows read as columns instead of stacking. A real per-column width pass
    // is a later refinement.
    "table{display:block;margin:8px 0}",
    "tr{display:flex}",
    "td,th{display:block;flex:1;padding:4px 8px;border:1px solid #3a4652}",
    "th{font-weight:bold;text-align:left}",
    "p,pre,blockquote{display:block;margin:10px 0}",
    "ul,ol{display:block;margin:10px 0;padding-left:28px}",
    "pre,code,kbd,samp,tt{font-family:monospace}",
    "pre{white-space:pre;display:block;margin:10px 0}textarea{white-space:pre}",
    "hr{display:block;margin:8px 0}",
    "input,textarea,select,button{display:block;margin:6px 0;padding:6px 8px;",
    "border:1px solid #6b7885;background:#10161b}",
    "input,select{height:34px}textarea{height:80px}",
    "button{background:#2b3844;border:1px solid #46a6b2}",
    "h1{display:block;font-weight:bold;font-size:30px;margin:13px 0}",
    "h2{display:block;font-weight:bold;font-size:24px;margin:12px 0}",
    "h3{display:block;font-weight:bold;font-size:19px;margin:10px 0}",
    "h4{display:block;font-weight:bold;font-size:16px;margin:9px 0}",
    "h5{display:block;font-weight:bold;font-size:13px;margin:8px 0}",
    "h6{display:block;font-weight:bold;font-size:12px;margin:8px 0}",
);

pub fn ua_rules() -> Vec<Rule> {
    parse(UA)
}
