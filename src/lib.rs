use comrak::Options;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn markdown_to_html(markdown_string: String) -> String {
    let mut options = Options::default();
    // All the GFM settings
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    let html = comrak::markdown_to_html(&markdown_string, &options);
    html
}
