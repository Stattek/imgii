/// Holds the image types that PNGII can output.
/// Each value holds an index into the `IMAGE_STR_TYPES` array.
pub enum OutputImageType {
    Png,
    Gif,
}

// image type string defines
const IMG_TYPE_PNG: &'static str = ".png";
const IMG_TYPE_GIF: &'static str = ".gif";

/// All image types stored in an array, for iterating through all image types.
pub const IMG_TYPES_ARRAY: &[&'static str] = &[IMG_TYPE_PNG, IMG_TYPE_GIF];

impl OutputImageType {
    /// Converts a string slice to an `OutputImageType`.
    ///
    /// * `output_image_type_str`:
    fn from_str(output_image_type_str: &str) -> Option<Self> {
        match output_image_type_str {
            IMG_TYPE_PNG => Some(OutputImageType::Png),
            IMG_TYPE_GIF => Some(OutputImageType::Gif),
            _ => None,
        }
    }

    /// Converts this file name to an `OutputImageType`.
    ///
    /// * `file_name`: The file name to check the file extension of.
    pub fn from_file_name(file_name: &str) -> Option<Self> {
        // find where the file extension starts (last "." in the string)
        let file_extension_start_idx = file_name.rfind(".");
        match file_extension_start_idx {
            Some(file_extension_start_idx) => {
                // match the string, starting from the last "."
                // see if this file extension matches any of the file extension strings we have
                Self::from_str(&file_name[file_extension_start_idx..])
            }
            None => None,
        }
    }

    /// Converts to this type's file extension.
    pub fn as_file_extension(&self) -> &'static str {
        match *self {
            OutputImageType::Png => IMG_TYPE_PNG,
            OutputImageType::Gif => IMG_TYPE_GIF,
        }
    }
}

/// Holds whether the program should convert a batch of inputs or just a single.
#[derive(PartialEq, Eq)]
pub enum ImageBatchType {
    Single,
    /// Contains the final index for this batch
    BatchWithFinalIdx(u32),
}
