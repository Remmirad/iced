//! Access the clipboard.
use iced_core::clipboard::ClipboardContent;

use crate::core::clipboard::Kind;
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

/// A clipboard action to be performed by some [`Task`].
///
/// [`Task`]: crate::Task
#[derive(Debug)]
pub enum Action {
    /// Read the clipboard and produce `T` with the result.
    Read {
        /// The clipboard target.
        target: Kind,
        /// The channel to send the read contents.
        channel: oneshot::Sender<Option<String>>,
    },

    /// Read the clipboard and produce `T` with the result.
    ReadContent {
        /// The channel to send the read contents.
        channel: oneshot::Sender<Result<ClipboardContent, String>>,
    },

    /// Write the given contents to the clipboard.
    Write {
        /// The clipboard target.
        target: Kind,
        /// The contents to be written.
        contents: String,
    },

    /// Write the given contents to the clipboard.
    WriteContent {
        /// Content
        contents: ClipboardContent,
    },
}

/// Read the current contents of the clipboard.
pub fn read() -> Task<Option<String>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            target: Kind::Standard,
            channel,
        })
    })
}

/// Read the current contents of the clipboard.
pub fn read_content() -> Task<Result<ClipboardContent, String>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::ReadContent { channel })
    })
}

/// Read the current contents of the primary clipboard.
pub fn read_primary() -> Task<Option<String>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            target: Kind::Primary,
            channel,
        })
    })
}

/// Write the given contents to the clipboard.
pub fn write<T>(contents: String) -> Task<T> {
    task::effect(crate::Action::Clipboard(Action::Write {
        target: Kind::Standard,
        contents,
    }))
}

/// Write the given contents to the clipboard.
pub fn write_content<T>(contents: ClipboardContent) -> Task<T> {
    task::effect(crate::Action::Clipboard(Action::WriteContent { contents }))
}

/// Write the given contents to the primary clipboard.
pub fn write_primary<Message>(contents: String) -> Task<Message> {
    task::effect(crate::Action::Clipboard(Action::Write {
        target: Kind::Primary,
        contents,
    }))
}
