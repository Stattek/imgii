//! Error implementation for `imgii` errors.

use thiserror::Error;

/// An error that can be returned by Imgii. Represents errors when converting images.
#[derive(Error, Debug)]
pub enum ImgiiError {
    /// Errors related to fonts.
    #[error("{0}")]
    Font(#[from] FontError),
    /// Error related to parsing ASCII.
    #[error("{0}")]
    Parse(#[from] ParseError),
    /// Error related to image.
    #[error("{0}")]
    Image(#[from] ImageError),
    /// I/O operation error.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
    /// Invalid argument error.
    #[error("invalid argument(s) provided")]
    InvalidArgument,
    /// Unknown, unspecified internal error.
    #[error("an internal error has occurred")]
    Internal,
}

#[derive(Debug, Error, Clone)]
pub enum FontError {
    #[error("could not load font {font_name}")]
    FontLoad { font_name: String },
}

/// ASCII text parsing error. Use this when parsing ASCII text and something goes wrong.
///
/// Suberror of [`ImgiiError`].
#[derive(Error, Debug, Clone)]
pub enum ParseError {
    /// Handles regex errors.
    #[error("imgii regular expression failed ({0})")]
    Regex(#[from] regex::Error),
    /// Handles errors related to parsing values.
    #[error("could not parse int value from string due to {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    /// Handles errors related to parsing colors from colored output
    #[error("could not parse value {value_name} from string ({the_str}), parse error ({err})")]
    ParseColor {
        /// The name of the value to parse.
        value_name: String,
        /// The string that parsing was attempted on but failed.
        the_str: String,
        /// The `std::num::ParseIntError` that was emitted upon failure to parse.
        err: std::num::ParseIntError,
    },
}

/// Represents an error while creating an image.
///
/// Suberror of [`ImgiiError`].
#[derive(Error, Debug, Clone)]
pub enum ImageError {
    #[error("invalid parameter {parameter_name} found")]
    InvalidParameter { parameter_name: String },
    #[error("parsing error found at row {image_row_number} of the image")]
    ParseImage {
        /// The row number of the image where this occurred.
        image_row_number: usize,
    },
    #[error("rendering failed because {reason}")]
    Render {
        /// The reason for the render error. Since this error is intended to handle various internals
        /// that aren't well represented by errors, this will explain why the error ocurred.
        reason: String,
    },
}

/*
 * NOTE: Implement any `From` traits here.
 */

// NOTE:
// ImgiiError should only have to implement From for errors for convenience. This avoids having to
// map_err from one error to another.
impl From<regex::Error> for ImgiiError {
    fn from(err: regex::Error) -> Self {
        Self::Parse(ParseError::Regex(err))
    }
}
