use std::{error::Error, fmt::Display};

/*
* NOTE: Struct definitions go below.
*/

/// An error that can be returned by Imgii. Represents errors when converting images.
#[derive(Debug)]
pub enum ImgiiError {
    /// Errors related to fonts.
    Font(FontError),
    /// Error related to parsing ASCII.
    Parse(ParseError),
    /// Unknown, unspecified internal error.
    Internal(InternalError),
    /// Error related to image.
    Image(ImageError),
    /// I/O operation error.
    Io(std::io::Error),
    Other(OtherError),
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

/// Represents an error while creating an image.
///
/// Suberror of [`ImgiiError`].
#[derive(Debug, Clone)]
pub enum ImageError {
    InvalidParameter(InvalidParameterError),
    ParseImage(ParseImageError),
}

/// Contains other errors. These are errors that can be emitted from other crates for various
/// reasons.
///
/// Suberror of [`ImgiiError`].
#[derive(Debug)]
pub struct OtherError {
    // we can hold any other Error in here
    other_err: Box<dyn Error>,
}

/// Represents an invalid parameter error when creating image.
///
/// Suberror of [`ImageError`].
#[derive(Debug, Clone)]
pub struct InvalidParameterError {
    parameter_name: String,
}

/// Represents an error that occurred while parsing an image (in a 2D fashion).
///
/// Suberror of [`ImageError`].
#[derive(Debug, Clone)]
pub struct ParseImageError {
    /// The row number of the image where this occurred.
    image_row_number: usize,
}

/*
 * NOTE: Implement `Display` below for errors that are intended to also implement Error.
 */

impl Display for ImgiiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImgiiError::Font(font_error) => {
                write!(f, "{font_error}")
            }
            ImgiiError::Parse(parse_error) => {
                write!(f, "{parse_error}")
            }
            ImgiiError::Internal(internal_error) => {
                write!(f, "{internal_error}")
            }
            ImgiiError::Io(io_error) => {
                write!(f, "{io_error}")
            }
            ImgiiError::Other(other_error) => {
                write!(f, "{other_error}")
            }
            ImgiiError::Image(image_error) => {
                write!(f, "{image_error}")
            }
        }
    }
}

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

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "an internal error has occurred")
    }
}

impl Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::InvalidParameter(invalid_parameter_error) => {
                write!(
                    f,
                    "invalid parameter {} found",
                    invalid_parameter_error.parameter_name
                )
            }
            ImageError::ParseImage(parse_image_error) => {
                write!(
                    f,
                    "parsing error found at row {} of the image",
                    parse_image_error.image_row_number
                )
            }
        }
    }
}

impl Display for OtherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an error from another crate/boxed error occurred ({})",
            self.other_err
        )
    }
}

/*
 * NOTE: Implement Error for error types below.
 */

// we don't need to implement anything since there are default implementations for this trait
impl Error for FontError {}
impl Error for ParseError {}
impl Error for ImgiiError {}
impl Error for OtherError {}

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

impl From<std::io::Error> for ImgiiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ImageError> for ImgiiError {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}

impl From<InvalidParameterError> for ImgiiError {
    fn from(value: InvalidParameterError) -> Self {
        Self::Image(ImageError::InvalidParameter(value))
    }
}

impl From<ParseImageError> for ImgiiError {
    fn from(value: ParseImageError) -> Self {
        Self::Image(ImageError::ParseImage(value))
    }
}

// for converting from errors boxed at runtime
impl From<Box<dyn Error>> for ImgiiError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::Other(OtherError::new(value))
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

impl OtherError {
    /// Creates a new [`OtherError`] from a boxed error (created at runtime).
    ///
    /// For use with other kinds of errors that the program can handle.
    ///
    /// * `other_err`: The other error, boxed.
    pub fn new(other_err: Box<dyn Error>) -> Self {
        Self { other_err }
    }
}

impl InvalidParameterError {
    /// Creates a new [`InvalidParameterError`].
    ///
    /// * `parameter_name`: The parameter name(s) that was invalid.
    #[must_use]
    pub fn new(parameter_name: String) -> Self {
        Self { parameter_name }
    }
}

impl ParseImageError {
    /// Creates a new [`ParseImageError`].
    ///
    /// * `image_row_number`: The image row number.
    pub fn new(image_row_number: usize) -> Self {
        Self { image_row_number }
    }
}
