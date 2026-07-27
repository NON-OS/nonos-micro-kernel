use std::fmt;
use std::fmt::Formatter;

use crate::RawOsStr;

pub(crate) use super::wtf8::ends_with;
pub(crate) use super::wtf8::starts_with;

if_nightly! {
    use std::os::windows::ffi::OsStrExt;
}

if_not_nightly! {
    pub(crate) use crate::util::is_continuation;

    use super::wtf8;
    use super::wtf8::CodePoints;
    use super::Result;
}

if_not_nightly! {
    pub(crate) fn validate_bytes(string: &[u8]) -> Result<()> {
        wtf8::encode_wide(string).try_for_each(|x| x.map(drop))
    }
}

#[cfg_attr(not(feature = "nightly"), allow(deprecated))]
pub(crate) fn encode_wide(
    string: &RawOsStr,
) -> impl '_ + Iterator<Item = u16> {
    if_nightly_return! {
        {
            string.as_os_str().encode_wide()
        }
        wtf8::encode_wide(string.as_raw_bytes()).map(|x| expect_encoded!(x))
    }
}

if_not_nightly! {
    pub(crate) fn decode_code_point(string: &[u8]) -> u32 {
        let mut code_points = CodePoints::new(string.iter().copied());
        let code_point = expect_encoded!(code_points
            .next()
            .expect("cannot parse code point from empty string"));
        assert_eq!(None, code_points.next(), "multiple code points found");
        code_point
    }
}

pub(crate) fn debug(string: &RawOsStr, f: &mut Formatter<'_>) -> fmt::Result {
    for wchar in encode_wide(string) {
        write!(f, "\\u{{{:X}}}", wchar)?;
    }
    Ok(())
}

#[cfg(feature = "uniquote")]
pub(crate) mod uniquote {
    use uniquote::Formatter;
    use uniquote::Result;

    use crate::RawOsStr;

    pub(crate) fn escape(string: &RawOsStr, f: &mut Formatter<'_>) -> Result {
        f.escape_utf16(super::encode_wide(string))
    }
}
