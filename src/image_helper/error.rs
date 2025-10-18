use std::{error::Error, fmt::Display};

use image::error;

/*
* NOTE: Struct definitions go below.
*/

/// An error that can be returned by Imgii. Represents errors when converting images.
#[derive(Debug, Clone)]
pub enum ImgiiError {
    Font(FontError),
    Parse(ParseError),
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

impl From<FontError> for ImgiiError {
    fn from(err: FontError) -> Self {
        Self::Font(err)
    }
}

// for converting from a regular expression error
impl From<regex::Error> for ImgiiError {
    fn from(err: regex::Error) -> Self {
        Self::Parse(ParseError::Regex(err))
    }
}

impl From<ParseIntError> for ImgiiError {
    fn from(err: ParseIntError) -> Self {
        Self::Parse(ParseError::ParseValue(err))
    }
}

/*
 * NOTE: Add any custom implementation blocks for errors below.
 */

impl FontError {
    /// Creates a new [`FontError`].
    ///
    /// * `font_name`: The font file name which failed to be created.
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
    pub fn new(value_name: String, the_str: String, err: std::num::ParseIntError) -> Self {
        Self {
            value_name,
            the_str,
            err,
        }
    }
}
