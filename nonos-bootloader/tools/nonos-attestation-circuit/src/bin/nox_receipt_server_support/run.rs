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

use std::io::Read;

use tiny_http::{Header, Method, Response, Server};

use super::args::parse;
use super::handle::handle;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    std::fs::create_dir_all(&args.spool)?;
    let server = Server::http(&args.listen).map_err(|e| format!("bind {}: {e}", args.listen))?;
    eprintln!("nox-receipt-server listening on {}", args.listen);
    let json =
        Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).expect("static header");
    for mut request in server.incoming_requests() {
        let response = match (request.method(), request.url()) {
            (Method::Post, "/submit") => {
                let mut body = Vec::new();
                let read =
                    request.as_reader().take(args.max_bytes as u64 + 1).read_to_end(&mut body);
                match read {
                    Ok(n) if n > args.max_bytes => {
                        reply(413, "{\"accepted\":false,\"error\":\"submission too large\"}")
                    }
                    Ok(_) => match handle(&body, &args) {
                        Ok(ok) => reply(200, &ok),
                        Err(e) => reply(
                            422,
                            &format!(
                                "{{\"accepted\":false,\"error\":{}}}",
                                serde_json::to_string(&e).unwrap_or_else(|_| "\"rejected\"".into())
                            ),
                        ),
                    },
                    Err(e) => reply(400, &format!("{{\"accepted\":false,\"error\":\"{e}\"}}")),
                }
            }
            (Method::Get, "/health") => reply(200, "{\"ok\":true}"),
            _ => reply(404, "{\"error\":\"not found\"}"),
        };
        let _ = request.respond(response.with_header(json.clone()));
    }
    Ok(())
}

fn reply(code: u32, body: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(body).with_status_code(code as u16)
}
