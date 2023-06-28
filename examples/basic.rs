use console::style;

fn main() -> std::io::Result<()> {
    claquer::clear_screen()?;

    claquer::intro(style(" create-app ").on_cyan().black())?;

    claquer::group(vec![claquer::item("path", |_| {
        claquer::text("Where should we create your project?")
            .placeholder("./sparkling-solid")
            .validate(|input: &str| {
                if input.is_empty() {
                    Err("Please enter a path.".into())
                } else if !input.starts_with("./") {
                    Err("Please enter a relative path".into())
                } else {
                    Ok(())
                }
            })
            .interact()
    })])?;

    claquer::outro(format!(
        "Problems? {}",
        style("https://example.com/issues").cyan().underlined()
    ))?;

    Ok(())
}
