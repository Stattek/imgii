use font_loader::system_fonts;

pub fn list_fonts() -> Vec<String> {
    let mut property = system_fonts::FontPropertyBuilder::new().monospace().build();
    system_fonts::query_specific(&mut property)
}

pub fn load_monospace_font(font_name: &str) -> Option<(Vec<u8>, i32)> {
    let property = system_fonts::FontPropertyBuilder::new()
        .monospace()
        .family(font_name)
        .build();
    system_fonts::get(&property)
}
