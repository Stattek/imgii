//! The options for using imgii.

// We need to re-export these, as they might be necessary for users of this library. Imgii's CLI
// uses these.
pub use rascii_art::RenderOptions as RasciiOptions;
pub use rascii_art::{
    charsets::{Charset, from_enum, to_charset_enum},
    convert_string_to_str_vec,
};

const DEFAULT_CHAR_FONT_SIZE: u32 = 16;

/// Options for creating the output ASCII PNG.
#[derive(Debug, Clone)]
pub struct ImgiiOptions {
    /// The font size of the output image.
    font_size: u32,

    /// Sets a black background behind the image.
    ///
    /// No background by default.
    background: bool,
}

impl ImgiiOptions {
    /// Creates a new image options object.
    #[must_use]
    fn new(font_size: u32, background: bool) -> Self {
        Self {
            font_size,
            background,
        }
    }

    /// Gets the font size to use to generate the image.
    #[must_use]
    pub fn font_size(&self) -> u32 {
        self.font_size
    }

    /// Gets the background flag. If true, sets a background behind the output image.
    pub fn background(&self) -> bool {
        self.background
    }
}

/// Builder for [`ImgiiOptions`]. Intended way to create options for Imgii.
#[derive(Debug, Clone)]
pub struct ImgiiOptionsBuilder {
    /// The font size of the output image.
    font_size: u32,

    /// Whether to set a background behind the image.
    background: bool,
}

impl Default for ImgiiOptionsBuilder {
    fn default() -> Self {
        Self {
            font_size: DEFAULT_CHAR_FONT_SIZE,
            background: false,
        }
    }
}

impl ImgiiOptionsBuilder {
    /// Creates a new builder with defaults. Behaves the same as calling
    /// [`ImgiiOptionsBuilder::default()`].
    pub fn new() -> ImgiiOptionsBuilder {
        Self::default()
    }

    /// Sets the font size of the output [`ImgiiOptions`].
    ///
    /// * `font_size`: The font size.
    pub fn font_size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the background flag for the output [`ImgiiOptions`]
    ///
    /// * `background`: The background flag.
    pub fn background(mut self, background: bool) -> Self {
        self.background = background;
        self
    }

    /// Builds a new [`ImgiiOptions`] instance from chosen values in this builder.
    pub fn build(&self) -> ImgiiOptions {
        ImgiiOptions::new(self.font_size, self.background)
    }
}
