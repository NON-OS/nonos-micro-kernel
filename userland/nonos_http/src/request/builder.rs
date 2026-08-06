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
//! Setting up a request.

extern crate alloc;

use super::build::RequestBuilder;

impl<'a> RequestBuilder<'a> {
    /// A GET for `target` on `host`.
    pub fn get(host: &'a str, target: &'a str) -> RequestBuilder<'a> {
        RequestBuilder {
            method: "GET",
            target,
            host,
            user_agent: "git/2.43",
            accept: "*/*",
            content_type: None,
            body: &[],
        }
    }

    /// A POST carrying `body` as `content_type`.
    pub fn post(
        host: &'a str,
        target: &'a str,
        content_type: &'a str,
        body: &'a [u8],
    ) -> RequestBuilder<'a> {
        RequestBuilder {
            method: "POST",
            target,
            host,
            user_agent: "git/2.43",
            accept: "*/*",
            content_type: Some(content_type),
            body,
        }
    }

    /// How the client names itself. Git servers vary what they advertise by
    /// user agent, so this is worth being able to set rather than fixed.
    pub fn user_agent(mut self, agent: &'a str) -> RequestBuilder<'a> {
        self.user_agent = agent;
        self
    }

    pub fn accept(mut self, accept: &'a str) -> RequestBuilder<'a> {
        self.accept = accept;
        self
    }
}
