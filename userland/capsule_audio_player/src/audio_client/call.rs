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

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};
use super::proto::{close_request, feed_request, open_request, pause_request, read_status, read_stream_id, E_AGAIN};

const SERVICE_NAME: &[u8] = b"audio.server";
const REQUEST_ID: u32 = 1;
const CALL_TIMEOUT_MS: u64 = 1000;

pub enum FeedResult { Fed, WouldBlock }

pub struct AudioClient { port: u64, stream_id: u32 }

impl AudioClient {
    pub fn connect() -> Result<AudioClient, &'static str> {
        let mut port = 0u32;
        let mut pid = 0u32;
        let rc = mk_service_lookup(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), &mut port, &mut pid);
        if rc < 0 || port == 0 {
            return Err("audio.server lookup failed");
        }
        Ok(AudioClient { port: port as u64, stream_id: 0 })
    }

    fn round_trip(&self, req: &[u8]) -> ([u8; 32], i64) {
        let mut resp = [0u8; 32];
        let rc = mk_ipc_call_timeout(self.port, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len(), CALL_TIMEOUT_MS);
        (resp, rc)
    }
    pub fn open(&mut self, format: u16) -> Result<u32, &'static str> {
        let (resp, rc) = self.round_trip(&open_request(REQUEST_ID, format));
        if rc < 28 {
            return Err("audio.server open: no reply");
        }
        if read_status(&resp[..rc as usize]) < 0 {
            return Err("audio.server open rejected");
        }
        self.stream_id = read_stream_id(&resp[..rc as usize]);
        Ok(self.stream_id)
    }
    pub fn feed(&mut self, pcm: &[i16]) -> Result<FeedResult, &'static str> {
        if pcm.is_empty() || pcm.len() % 2 != 0 {
            return Err("audio.server feed: invalid pcm length");
        }
        let (resp, rc) = self.round_trip(&feed_request(REQUEST_ID, self.stream_id, pcm));
        if rc < 24 {
            return Err("audio.server feed: no reply");
        }
        match read_status(&resp[..rc as usize]) {
            s if s == E_AGAIN => Ok(FeedResult::WouldBlock),
            s if s < 0 => Err("audio.server feed rejected"),
            _ => Ok(FeedResult::Fed),
        }
    }
    pub fn pause(&mut self) {
        self.round_trip(&pause_request(REQUEST_ID, self.stream_id));
    }
    pub fn close(&mut self) {
        self.round_trip(&close_request(REQUEST_ID, self.stream_id));
    }
}
