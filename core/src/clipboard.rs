//! Access the clipboard.

use std::{error::Error, path::PathBuf};

/// ClipboardContent
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    /// String.
    String(String),
    /// File paths.
    Paths(Vec<PathBuf>),
    /// Cbor data.
    Cbor(Vec<u8>),
    /// Image in bmp format.
    BmpImage(Vec<u8>),
    /// Image in png format.
    PngImage(Vec<u8>),
    /// Image in jpeg format.
    JpegImage(Vec<u8>),
}

/// A buffer for short-term storage and transfer within and between
/// applications.
pub trait Clipboard {
    /// Reads the current content of the [`Clipboard`] as text.
    fn read(&self, kind: Kind) -> Option<String>;

    ///
    fn read_content(&self) -> Result<ClipboardContent, Box<dyn Error>>;

    /// Writes the given text contents to the [`Clipboard`].
    fn write(&mut self, kind: Kind, contents: String);

    ///
    fn write_content(
        &mut self,
        contents: ClipboardContent,
    ) -> Result<(), Box<dyn Error>>;
}

/// The kind of [`Clipboard`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// The standard clipboard.
    Standard,
    /// The primary clipboard.
    ///
    /// Normally only present in X11 and Wayland.
    Primary,
}

/// A null implementation of the [`Clipboard`] trait.
#[derive(Debug, Clone, Copy)]
pub struct Null;

impl Clipboard for Null {
    fn read(&self, _kind: Kind) -> Option<String> {
        None
    }

    fn write(&mut self, _kind: Kind, _contents: String) {}

    fn read_content(&self) -> Result<ClipboardContent, Box<dyn Error>> {
        Err("Null".into())
    }

    fn write_content(
        &mut self,
        _contents: ClipboardContent,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
