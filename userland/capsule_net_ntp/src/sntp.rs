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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SntpError {
    Malformed,
    NotServer,
    BadTimestamp,
}

const NTP_UNIX_DELTA_SECS: u64 = 2_208_988_800;

pub fn build_request() -> [u8; 48] {
    let mut b = [0u8; 48];
    b[0] = 0x23;
    b
}

pub fn parse_reply(buf: &[u8]) -> Result<u64, SntpError> {
    if buf.len() != 48 {
        return Err(SntpError::Malformed);
    }
    let mode = buf[0] & 0x07;
    let stratum = buf[1];
    if mode != 4 || stratum == 0 || stratum > 15 {
        return Err(SntpError::NotServer);
    }
    let secs = u32::from_be_bytes([buf[40], buf[41], buf[42], buf[43]]) as u64;
    if secs < NTP_UNIX_DELTA_SECS {
        return Err(SntpError::BadTimestamp);
    }
    Ok((secs - NTP_UNIX_DELTA_SECS) * 1000)
}

#[cfg(test)]
mod tests {
    use super::{build_request, parse_reply, SntpError};

    #[test]
    fn request_is_client_mode_v4() {
        let r = build_request();
        assert_eq!(r.len(), 48);
        assert_eq!(r[0] & 0x38, 0x20);
        assert_eq!(r[0] & 0x07, 0x03);
    }

    #[test]
    fn parse_rejects_short_or_nonserver() {
        assert!(matches!(parse_reply(&[0u8; 10]), Err(SntpError::Malformed)));
        let mut b = [0u8; 48];
        b[0] = 0x23;
        assert!(matches!(parse_reply(&b), Err(SntpError::NotServer)));
    }

    #[test]
    fn parse_extracts_unix_ms() {
        let mut b = [0u8; 48];
        b[0] = 0x24;
        b[1] = 1;
        let secs: u32 = 3_913_056_000;
        b[40..44].copy_from_slice(&secs.to_be_bytes());
        assert!(matches!(parse_reply(&b), Ok(1_704_067_200_000)));
    }
}
