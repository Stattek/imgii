use std::ops::Deref;

use image::ImageBuffer;

/// Represents the image data to work with.
/// Holds an `ImageBuffer` with the image data.
#[derive(Clone)]
pub struct ImageData(ImageBuffer<image::Rgba<u8>, Vec<u8>>);

impl ImageData {
    /// Create a new ImageData struct as this image buffer.
    pub fn new(image_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self {
        Self(image_buffer)
    }
}

impl Deref for ImageData {
    type Target = ImageBuffer<image::Rgba<u8>, Vec<u8>>;

    /// Gets the underlying `ImageBuffer<image::Rgba<u8>, Vec<u8>>`
    /// that this struct is a wrapper for.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Simple conversions to make it possible to convert to and from an ImageData
// and its inner held type.
impl From<ImageBuffer<image::Rgba<u8>, Vec<u8>>> for ImageData {
    fn from(value: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self {
        Self(value)
    }
}

impl Into<ImageBuffer<image::Rgba<u8>, Vec<u8>>> for ImageData {
    fn into(self) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        self.0
    }
}
