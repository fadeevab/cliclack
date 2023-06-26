use std::io::Result;

use console::style;

fn main() -> Result<()> {
    claquer::clear_screen()?;

    claquer::intro(style(" create-app ").on_cyan().black())?;

    claquer::group(vec![claquer::item("path", |_| {
        claquer::text("Where should we create your project?")
            .default("./sparkling-solid".into())
            .validate_with(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a path.")
                } else if !input.starts_with("./") {
                    Err("Please enter a relative path")
                } else {
                    Ok(())
                }
            })
            .interact_text()
    })])?;

    claquer::outro(format!(
        "Problems? {}",
        style("https://example.com/issues").cyan().underlined()
    ))?;

    Ok(())
}
