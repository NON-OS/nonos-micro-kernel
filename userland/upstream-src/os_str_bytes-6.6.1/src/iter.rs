//! Iterators provided by this crate.

#![cfg_attr(os_str_bytes_docs_rs, doc(cfg(feature = "raw_os_str")))]

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::iter::FusedIterator;
use std::mem;

use super::pattern::Encoded;
use super::Pattern;
use super::RawOsStr;

// [memchr::memmem::FindIter] is not currently used, since this struct would
// become self-referential. Additionally, that iterator does not implement
// [DoubleEndedIterator], and its implementation would likely require
// significant changes to implement that trait.
/// The iterator returned by [`RawOsStr::split`].
pub struct RawSplit<'a, P>
where
    P: Pattern,
{
    string: Option<&'a RawOsStr>,
    pat: P::__Encoded,
}

impl<'a, P> RawSplit<'a, P>
where
    P: Pattern,
{
    #[track_caller]
    pub(super) fn new(string: &'a RawOsStr, pat: P) -> Self {
        let pat = pat.__encode();
        assert!(
            !pat.__get().is_empty(),
            "cannot split using an empty pattern",
        );
        Self {
            string: Some(string),
            pat,
        }
    }
}

macro_rules! impl_next {
    ( $self:ident , $split_method:ident , $swap:expr ) => {{
        $self
            .string?
            .$split_method(&$self.pat)
            .map(|(mut substring, mut string)| {
                if $swap {
                    mem::swap(&mut substring, &mut string);
                }
                $self.string = Some(string);
                substring
            })
            .or_else(|| $self.string.take())
    }};
}

impl<P> Clone for RawSplit<'_, P>
where
    P: Pattern,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            string: self.string,
            pat: self.pat.clone(),
        }
    }
}

impl<P> Debug for RawSplit<'_, P>
where
    P: Pattern,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawSplit")
            .field("string", &self.string)
            .field("pat", &self.pat)
            .finish()
    }
}

impl<P> DoubleEndedIterator for RawSplit<'_, P>
where
    P: Pattern,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        impl_next!(self, rsplit_once_raw, true)
    }
}

impl<P> FusedIterator for RawSplit<'_, P> where P: Pattern {}

impl<'a, P> Iterator for RawSplit<'a, P>
where
    P: Pattern,
{
    type Item = &'a RawOsStr;

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn next(&mut self) -> Option<Self::Item> {
        impl_next!(self, split_once_raw, false)
    }
}

/// A temporary type alias providing backward compatibility.
#[deprecated(since = "6.6.0", note = "use `RawSplit` instead")]
pub type Split<'a, P> = RawSplit<'a, P>;
