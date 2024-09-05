use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::Options;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn markdown_to_html(markdown_string: String) -> String {
    let mut plugins = comrak::Plugins::default();

    let syntax_highlighter = SyntectAdapterBuilder::new().css().build();
    plugins.render.codefence_syntax_highlighter = Some(&syntax_highlighter);

    let mut options = Options::default();
    // All the GFM settings
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    // Front matter
    options.extension.front_matter_delimiter = Some("---".to_string());

    let html = comrak::markdown_to_html_with_plugins(&markdown_string, &options, &plugins);
    html
}
