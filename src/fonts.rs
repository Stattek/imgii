use font_loader::system_fonts;

/// Lists all of the fonts that are installed on the system.
pub fn list_fonts() -> Vec<String> {
    let mut property = system_fonts::FontPropertyBuilder::new().monospace().build();
    system_fonts::query_specific(&mut property)
}

/// Loads the specified monospace font from the system.
///
/// * `font_name`: The name of the font.
///
/// # Returns
/// `Option<(Vec<u8>, i32)>` containing the bytes of the font, followed by the index of the font.
pub fn load_monospace_font(font_name: &str) -> Option<(Vec<u8>, i32)> {
    let property = system_fonts::FontPropertyBuilder::new()
        .monospace()
        .family(font_name)
        .build();
    system_fonts::get(&property)
}
