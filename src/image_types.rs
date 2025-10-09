/// Holds the image types that this program can output.
/// Each value holds an index into the `IMAGE_STR_TYPES` array.
pub enum OutputImageType {
    Png = 0,
    Gif = 1,
}

/// Use the value of `OutputImageType` as the index to this array
pub const IMAGE_STR_TYPES: [&'static str; 2] = [".png", ".gif"];
