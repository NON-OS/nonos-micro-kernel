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
    "template{display:none}noscript{display:none}meta{display:none}link{display:none}",
    "a{color:#4c9aff;text-decoration:underline}b{font-weight:bold}strong{font-weight:bold}",
    "body{display:block;margin:8px}",
    "div,section,article,main,header,footer,nav,aside,figure,figcaption,form,button,",
    "thead,tbody,tfoot,li,dl,dt,dd{display:block}",
    // The obsolete <center> still appears on older layouts (Hacker News wraps
    // its whole page in one). Without a block role it defaults to inline, and a
    // table nested in an inline box is flattened into inline runs, collapsing
    // the page onto a single line.
    "center{display:block;text-align:center}",
    // Tables lay each row as a flex line and each cell as an equal share, so
    // rows read as columns instead of stacking. A real per-column width pass
    // is a later refinement.
    "table{display:block;margin:8px 0}",
    "tr{display:flex}",
    "td,th{display:block;flex:1;padding:4px 8px;border:1px solid #3a4652}",
    "th{font-weight:bold;text-align:left}",
    "p,pre{display:block;margin:10px 0}",
    // Blockquotes indent from both margins (the CSS default is 40px) and carry
    // a left rule, the conventional quote treatment, so a quotation reads as set
    // apart from the body text rather than flush with it.
    "blockquote{display:block;margin:12px 40px;padding-left:16px;border-left:4px solid #d0d7de}",
    "ul,ol{display:block;margin:10px 0;padding-left:28px}",
    "pre,code,kbd,samp,tt{font-family:monospace}",
    "pre{white-space:pre;display:block;margin:10px 0}textarea{white-space:pre}",
    "hr{display:block;margin:8px 0}",
    // Light form controls: a page is white by default, so a dark control with
    // the near-black default text renders as an unreadable dark-on-dark field.
    // White fields with dark text and a soft border match how forms look on the
    // light pages that make up most of the web.
    "input,textarea,select,button{display:block;margin:6px 0;padding:6px 8px;",
    "border:1px solid #c8ccd0;background:#ffffff;color:#1a1a1a}",
    "input,select{height:34px}textarea{height:80px}",
    "button{background:#f2f3f5;border:1px solid #c2c6cb;color:#1a1a1a}",
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
