use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::result;

#[cfg(all(target_vendor = "fortanix", target_env = "sgx"))]
use std::os::fortanix_sgx as os;
#[cfg(target_os = "hermit")]
use std::os::hermit as os;
#[cfg(target_os = "solid_asp3")]
use std::os::solid as os;
#[cfg(unix)]
use std::os::unix as os;
#[cfg(target_os = "wasi")]
use std::os::wasi as os;
#[cfg(target_os = "xous")]
use std::os::xous as os;

#[cfg(not(target_vendor = "nonos"))]
use os::ffi::OsStrExt;
#[cfg(not(target_vendor = "nonos"))]
use os::ffi::OsStringExt;

if_raw_str! {
    pub(super) mod raw;
}

pub(super) type EncodingError = Infallible;

pub(super) type Result<T> = result::Result<T, EncodingError>;

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn os_str_from_bytes(string: &[u8]) -> Result<Cow<'_, OsStr>> {
    Ok(Cow::Borrowed(OsStrExt::from_bytes(string)))
}

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn os_str_to_bytes(os_string: &OsStr) -> Cow<'_, [u8]> {
    Cow::Borrowed(OsStrExt::as_bytes(os_string))
}

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn os_string_from_vec(string: Vec<u8>) -> Result<OsString> {
    Ok(OsStringExt::from_vec(string))
}

#[cfg(not(target_vendor = "nonos"))]
pub(super) fn os_string_into_vec(os_string: OsString) -> Vec<u8> {
    OsStringExt::into_vec(os_string)
}

// NONOS OsStr is a raw byte string, so the stable OsStr encoded-bytes API is a
// lossless round trip and needs no os::unix::ffi extension traits. This keeps
// the shim buildable against the byte-based std without depending on the unix
// module being present.
#[cfg(target_vendor = "nonos")]
pub(super) fn os_str_from_bytes(string: &[u8]) -> Result<Cow<'_, OsStr>> {
    // SAFETY: every byte sequence is a valid encoding for the byte-based OsStr.
    Ok(Cow::Borrowed(unsafe { OsStr::from_encoded_bytes_unchecked(string) }))
}

#[cfg(target_vendor = "nonos")]
pub(super) fn os_str_to_bytes(os_string: &OsStr) -> Cow<'_, [u8]> {
    Cow::Borrowed(os_string.as_encoded_bytes())
}

#[cfg(target_vendor = "nonos")]
pub(super) fn os_string_from_vec(string: Vec<u8>) -> Result<OsString> {
    // SAFETY: as above; the bytes came from a NONOS byte string.
    Ok(unsafe { OsStr::from_encoded_bytes_unchecked(&string) }.to_os_string())
}

#[cfg(target_vendor = "nonos")]
pub(super) fn os_string_into_vec(os_string: OsString) -> Vec<u8> {
    os_string.into_encoded_bytes()
}
