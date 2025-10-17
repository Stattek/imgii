use image::ImageBuffer;

// easier to read
pub type InternalImage = ImageBuffer<image::Rgba<u8>, Vec<u8>>;

/// Represents the image data to work with.
/// Holds an `ImageBuffer` with the image data.
#[derive(Debug, Clone)]
pub struct ImageData(InternalImage);

impl ImageData {
    /// Create a new ImageData struct as this image buffer.
    pub fn new(image_buffer: InternalImage) -> Self {
        Self(image_buffer)
    }

    /// Gets a reference to the internal buffer for this image data.
    pub fn as_buffer(&self) -> &InternalImage {
        &self.0
    }
}

// Simple conversion to make it possible to convert to and from an ImageData
// and its inner held type.
impl From<ImageData> for ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    fn from(value: ImageData) -> Self {
        value.0
    }
}
