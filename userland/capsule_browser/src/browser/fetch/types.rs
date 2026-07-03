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

use crate::browser::tls13::flight::ClientFlight;
use crate::browser::url::Url;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    SocksHello,
    SocksMethod,
    SocksConnect,
    TlsHello,
    TlsFlight,
    TlsVerify,
    SendReq,
    ReadBody,
    Decrypt,
    Done,
    Error,
}

pub struct TlsCtx {
    pub cf: ClientFlight,
    pub flight: Vec<u8>,
    pub now: u64,
}

pub struct Fetch {
    pub url: Url,
    pub handle: u32,
    pub phase: Phase,
    pub buf: Vec<u8>,
    pub socks: Vec<u8>,
    pub tls: Option<TlsCtx>,
    pub idle: u32,
    pub started_ms: i64,
    pub error: Option<&'static str>,
    pub suppress: bool,
    // Set when this fetch services an <img> resource rather than a page
    // navigation; carries the absolute source url the raster is keyed by.
    pub image: Option<alloc::string::String>,
    pub last_check: usize,
    // Form body for a POST navigation; None sends a GET.
    pub post: Option<alloc::string::String>,
    // A script-issued fetch: the body goes to the page callback, not the
    // renderer.
    pub js_req: bool,
    // An external stylesheet fetch: the body is appended to the page CSS and
    // triggers a re-layout, not a navigation.
    pub css: bool,
}
