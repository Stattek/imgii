use std::{error::Error, fmt::Display};

use image::error;

/*
* NOTE: Struct definitions go below.
*/

/// An error that can be returned by Imgii. Represents errors when converting images.
#[derive(Debug, Clone)]
pub enum ImgiiError {
    /// Errors related to fonts.
    Font(FontError),
    /// Error related to parsing ASCII.
    Parse(ParseError),
    /// Unknown, unspecified internal error.
    Internal(InternalError),
    /// I/O operation error.
    Io(IoError),
}

/// Font error. Use this when something related to the font has gone wrong.
///
/// Suberror of [`ImgiiError`].
#[derive(Debug, Clone)]
pub struct FontError {
    font_name: String,
}

/// ASCII text parsing error. Use this when parsing ASCII text and something goes wrong.
///
/// Suberror of [`ImgiiError`].
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Handles regex errors.
    Regex(regex::Error),
    /// Handles errors related to parsing values.
    ParseValue(ParseIntError),
}

/// Regular expression compiler error.
/// Doesn't actually implement Error, as it is easier to implement functionality in the super
/// error, [`ParseError`].
///
/// Suberror of [`ParseError`].
#[derive(Debug, Clone)]
pub struct ParseIntError {
    /// The name of the value to parse.
    value_name: String,
    /// The string that parsing was attempted on but failed.
    the_str: String,
    /// The `std::num::ParseIntError` that was emitted upon failure to parse.
    err: std::num::ParseIntError,
}

/// Some other, unknown internal error.
///
/// Suberror of [`ImgiiError`].
#[derive(Debug, Clone)]
pub struct InternalError;

/// Contains IO errors.
///
/// Suberror of [`ImgiiError`].
#[derive(Debug, Clone)]
pub enum IoError {
    File(FileError),
}

/// File I/O error.
/// Doesn't actually implement Error, as it is easier to implement functionality in the super
/// error, [`IoError`].
///
/// Suberror of [`IoError`].
#[derive(Debug, Clone)]
pub struct FileError {
    /// The file name causing the error
    file_name: String,

    /// The type of IO error.
    io_err: std::io::ErrorKind, // Since std::io::Error doesn't implement the Clone trait, we'll use this.
}

/*
 * NOTE: Implement `Display` below for errors that are intended to also implement Error.
 */

impl Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not read font {}", self.font_name)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regex(err) => {
                // let's just print their error out with ours
                write!(f, "imgii regular expression failed ({})", err)
            }
            Self::ParseValue(err) => {
                write!(
                    f,
                    "could not parse value {} from string ({}), parse error ({})",
                    err.value_name, err.the_str, err.err
                )
            }
        }
    }
}

/*
 * NOTE: Implement Error for error types below.
 */

// we don't need to implement anything since there are default implementations for this trait
impl Error for FontError {}
impl Error for ParseError {}

/*
 * NOTE: Implement any `From` traits here.
 */

// NOTE:
// ImgiiError should only have to implement From for all of its direct suberrors, but Rust makes me
// do another From impl for the errors that can be converted into a suberror type too.
//
// The suberrors can implement From for anything that can be converted into them specifically, then
// for each of those, a simple From can be implemented for ImgiiError with a call to `.into()`,
// which will convert into the suberror type, then it should convert into the ImgiiError type.
//
// This makes it easier to maintain, as more errors are added. This pattern should be replicated for
// suberrors which have their own suberrors.
impl From<FontError> for ImgiiError {
    fn from(err: FontError) -> Self {
        Self::Font(err)
    }
}

impl From<ParseError> for ImgiiError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<InternalError> for ImgiiError {
    fn from(err: InternalError) -> Self {
        Self::Internal(err)
    }
}

impl From<IoError> for ImgiiError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

// for converting from a regular expression error (not ours)
impl From<regex::Error> for ParseError {
    fn from(err: regex::Error) -> Self {
        Self::Regex(err)
    }
}
impl From<regex::Error> for ImgiiError {
    fn from(err: regex::Error) -> Self {
        // since we already have a way to convert this error to a ParseError, we can go from that
        // error to an ImgiiError
        err.into()
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseValue(err)
    }
}
impl From<ParseIntError> for ImgiiError {
    fn from(err: ParseIntError) -> Self {
        err.into()
    }
}

// NOTE: no impl for From<std::io::Error> for IoError because that type can represent so many
// different errors.
impl From<FileError> for IoError {
    fn from(value: FileError) -> Self {
        Self::File(value)
    }
}
impl From<FileError> for ImgiiError {
    fn from(value: FileError) -> Self {
        value.into()
    }
}

/*
 * NOTE: Add any custom implementation blocks for errors below.
 */

impl FontError {
    /// Creates a new [`FontError`].
    ///
    /// * `font_name`: The font file name which failed to be created.
    #[must_use]
    pub fn new(font_name: String) -> Self {
        Self { font_name }
    }
}

impl ParseIntError {
    /// Creates a new [`ParseIntError`].
    ///
    /// * `value_name`: The value name to parse.
    /// * `the_str`: The string that parsing was attempted on.
    /// * `err`: The `std::num::ParseIntError` that was emitted.
    #[must_use]
    pub fn new(value_name: String, the_str: String, err: std::num::ParseIntError) -> Self {
        Self {
            value_name,
            the_str,
            err,
        }
    }
}

impl FileError {
    /// Creates a new [`FileError`].
    ///
    /// * `file_name`: The name of the file that caused the error.
    /// * `io_err`: The kind of I/O error.
    #[must_use]
    pub fn new(file_name: String, io_err: std::io::ErrorKind) -> Self {
        Self { file_name, io_err }
    }
}
