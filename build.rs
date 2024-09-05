use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufWriter, Error};
use std::path::{PathBuf};
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle};

// Example custom build script.
fn main() -> Result<(), Error> {
    let env_release_type = std::env::var("CARGO_MAKE_PROFILE").unwrap_or("development".to_string());

    let output_path = {
        if env_release_type == "production" {
            PathBuf::from("./pkg/")
        } else {
            PathBuf::from("./test_website/css/")
        }
    };
    fs::create_dir_all(&output_path)?;
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!(
        "cargo::rerun-if-changed={}",
        output_path.as_path().to_str().unwrap()
    );

    let css = "\
@import url(\"theme-light.css\") (prefers-color-scheme: light);
@import url(\"theme-dark.css\") (prefers-color-scheme: dark);

@media (prefers-color-scheme: dark) {
    body {
        background-color: gray;
    }
}
@media (prefers-color-scheme: light) {
    body {
        background-color: lightgray;
    }
}";

    let css_file = File::create(output_path.join("synhtml-css-classes.css"))?;
    let mut css_writer = BufWriter::new(&css_file);

    writeln!(css_writer, "{}", css)?;

    let ts = ThemeSet::load_defaults();

    // create dark color scheme css
    let dark_theme = &ts.themes["Solarized (dark)"];
    let css_dark_file = File::create(output_path.join("theme-dark.css"))?;
    let mut css_dark_writer = BufWriter::new(&css_dark_file);

    let css_dark = css_for_theme_with_class_style(dark_theme, ClassStyle::Spaced).unwrap();
    writeln!(css_dark_writer, "{}", css_dark)?;

    // create light color scheme css
    let light_theme = &ts.themes["Solarized (light)"];
    let css_light_file = File::create(output_path.join("theme-light.css"))?;
    let mut css_light_writer = BufWriter::new(&css_light_file);

    let css_light = css_for_theme_with_class_style(light_theme, ClassStyle::Spaced).unwrap();
    writeln!(css_light_writer, "{}", css_light)?;

    Ok(())
}
