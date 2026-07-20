// NONOS Operating System (AGPL-3.0-or-later)
//! The targeted forgeries the reject gate proves the assembly catches. Each
//! keeps its region internally consistent where possible, so the rejection
//! comes from the binding under attack rather than a local constraint.

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tamper {
    None,
    /// A DEEP trace value cut loose from its authenticated opening.
    ReboundTraceValue,
    /// The deep and comp values authenticated under each other's root.
    SwappedRoot,
    /// A DEEP batching coefficient the transcript never squeezed.
    OffTranscriptCoeff,
}
