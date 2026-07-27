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

use super::defs::{Fed, FeedSink};
use crate::audio_client::{AudioClient, FeedResult};

impl FeedSink for AudioClient {
    fn open(&mut self, format: u16) -> Result<(), &'static str> {
        self.open(format).map(|_| ())
    }

    fn feed(&mut self, pcm: &[i16]) -> Fed {
        match self.feed(pcm) {
            Ok(FeedResult::Fed) => Fed::Accepted,
            Ok(FeedResult::WouldBlock) => Fed::WouldBlock,
            Err(_) => Fed::WouldBlock,
        }
    }

    fn pause(&mut self) {
        AudioClient::pause(self);
    }

    fn resume(&mut self) {
        AudioClient::resume(self);
    }

    fn close(&mut self) {
        AudioClient::close(self);
    }
}
