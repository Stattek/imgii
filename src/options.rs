//! The options for using imgii.

// We need to re-export these, as they might be necessary for users of this library. Imgii's CLI
// uses these.
pub use rascii_art::RenderOptions as RasciiOptions;
pub use rascii_art::{
    charsets::{Charset, from_enum, to_charset_enum},
    convert_string_to_str_vec,
};

const DEFAULT_CHAR_FONT_SIZE: u32 = 16;

// NOTE: we don't want to ever make members of ImgiiOptions public so users can't cause imgii to
// crash by setting invalid options.

/// Options for creating the output ASCII PNG.
#[derive(Debug, Clone)]
pub struct ImgiiOptions<'a> {
    /// The font size of the output image.
    font_size: u32,

    /// Sets a black background behind the image.
    ///
    /// No background by default.
    background: bool,

    /// The RASCII options for converting an image to ASCII.
    rascii_options: RasciiOptions<'a>,
}

impl<'a> ImgiiOptions<'a> {
    /// Creates a new image options object.
    #[must_use]
    fn new(font_size: u32, background: bool, rascii_options: RasciiOptions<'a>) -> Self {
        Self {
            font_size,
            background,
            rascii_options,
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

    /// Gets the RASCII options.
    pub fn rascii_options(&self) -> &RasciiOptions<'a> {
        &self.rascii_options
    }
}

/// Builder for [`ImgiiOptions`]. Intended way to create options for Imgii.
#[derive(Debug, Clone)]
pub struct ImgiiOptionsBuilder<'a> {
    /// The font size of the output image.
    font_size: u32,

    /// Whether to set a background behind the image.
    background: bool,

    /// The RASCII options used under the hood to convert an image to ASCII.
    rascii_options: RasciiOptions<'a>,
}

impl<'a> Default for ImgiiOptionsBuilder<'a> {
    fn default() -> Self {
        Self {
            font_size: DEFAULT_CHAR_FONT_SIZE,
            background: false,
            rascii_options: RasciiOptions::default()
                .colored(true)
                .escape_each_colored_char(true),
        }
    }
}

impl<'a> ImgiiOptionsBuilder<'a> {
    /// Creates a new builder with defaults. Behaves the same as calling
    /// [`ImgiiOptionsBuilder::default()`].
    pub fn new() -> Self {
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
    pub fn build(&self) -> ImgiiOptions<'a> {
        ImgiiOptions::new(self.font_size, self.background, self.rascii_options.clone())
    }

    /*
     * RASCII options values
     * (that are allowed to be configured, since we need some of them to always be set so imgii
     * doesn't fail.)
     */

    /// Set the width of the rendered image.
    pub fn width(mut self, width: u32) -> Self {
        self.rascii_options.width = Some(width);
        self
    }

    /// Set the height of the rendered image.
    pub fn height(mut self, height: u32) -> Self {
        self.rascii_options.height = Some(height);
        self
    }

    /// Set whether the rendered image charset should be inverted.
    pub fn invert(mut self, invert: bool) -> Self {
        self.rascii_options.invert = invert;
        self
    }

    /// Set the charset to use for the rendered image.
    pub fn charset(mut self, charset: &'a [&'a str]) -> Self {
        self.rascii_options.charset = charset;
        self
    }

    /// Set the character override to repeat in the rendered image. Ignores the current charset.
    pub fn char_override(mut self, char_override: Vec<String>) -> Self {
        self.rascii_options.char_override = Some(char_override);
        self
    }
}
