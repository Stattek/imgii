use crate::image_helper::render_char_to_png::DEFAULT_CHAR_FONT_SIZE;

/// Options for creating the output ASCII PNG.
#[derive(Debug, Clone)]
pub struct ImgiiOptions {
    /// The font size of the output image.
    font_size: Option<u32>,

    /// Sets a black background behind the image.
    ///
    /// No background by default.
    pub background: bool,
}

impl ImgiiOptions {
    /// Creates a new image options object.
    pub fn new(font_size: Option<u32>, background: bool) -> Self {
        Self {
            font_size,
            background,
        }
    }

    /// Gets the font size if present, otherwise gives back the default
    /// font size.
    pub fn font_size(&self) -> u32 {
        self.font_size.unwrap_or(DEFAULT_CHAR_FONT_SIZE)
    }
}
