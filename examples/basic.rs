use std::{thread, time::Duration};

use console::style;

fn main() -> std::io::Result<()> {
    claquer::clear_screen()?;

    claquer::intro(style(" create-app ").on_cyan().black())?;

    let path: String = claquer::text("Where should we create your project?")
        .placeholder("./sparkling-solid")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a path.")
            } else if !input.starts_with("./") {
                Err("Please enter a relative path")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let _password = claquer::password("Provide a password")
        .mask('▪')
        .interact()?;

    let _kind = claquer::select(format!("Pick a project type within '{path}'"))
        .initial_value("ts")
        .item("ts", "TypeScript", "")
        .item("js", "JavaScript", "")
        .item("coffee", "CoffeeScript", "oh no")
        .interact()?;

    let _tools = claquer::multiselect("Select additional tools")
        .initial_values(vec!["prettier", "eslint"])
        .item("prettier", "Prettier", "recommended")
        .item("eslint", "ESLint", "recommended")
        .item("stylelint", "Stylelint", "")
        .item("gh-action", "GitHub Action", "")
        .interact()?;

    let _: u8 = claquer::text("Input a number (not greater than 256)").interact()?;

    let install = claquer::confirm("Install dependencies?").interact()?;

    if install {
        let mut spinner = claquer::spinner();
        spinner.start("Installing via pnpm");
        thread::sleep(Duration::from_secs(5));
        spinner.stop("Installed via pnpm");
    }

    let next_steps = format!(
        "{path}\n{pnpm_install}pnpm dev\n",
        pnpm_install = if install { "" } else { "pnpm install\n" }
    );

    claquer::note("Next steps.", next_steps)?;

    claquer::outro(format!(
        "Problems? {}",
        style("https://example.com/issues").cyan().underlined()
    ))?;

    Ok(())
}
