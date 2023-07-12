use std::{thread, time::Duration};

use console::style;

fn main() -> std::io::Result<()> {
    cliclack::clear_screen()?;

    cliclack::intro(style(" create-app ").on_cyan().black())?;

    let path: String = cliclack::input("Where should we create your project?")
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

    let _password = cliclack::password("Provide a password")
        .mask('▪')
        .interact()?;

    let _kind = cliclack::select(format!("Pick a project type within '{path}'"))
        .initial_value("ts")
        .item("ts", "TypeScript", "")
        .item("js", "JavaScript", "")
        .item("coffee", "CoffeeScript", "oh no")
        .interact()?;

    let _tools = cliclack::multiselect("Select additional tools")
        .initial_values(vec!["prettier", "eslint"])
        .item("prettier", "Prettier", "recommended")
        .item("eslint", "ESLint", "recommended")
        .item("stylelint", "Stylelint", "")
        .item("gh-action", "GitHub Action", "")
        .interact()?;

    let _: u8 = cliclack::input("Input a number (not greater than 256)").interact()?;

    let install = cliclack::confirm("Install dependencies?").interact()?;

    if install {
        let mut spinner = cliclack::spinner();
        spinner.start("Installing via pnpm");
        thread::sleep(Duration::from_secs(5));
        spinner.stop("Installed via pnpm");
    }

    let next_steps = format!(
        "{path}\n{pnpm_install}pnpm dev\n",
        pnpm_install = if install { "" } else { "pnpm install\n" }
    );

    cliclack::note("Next steps.", next_steps)?;

    cliclack::outro(format!(
        "Problems? {}\n",
        style("https://example.com/issues").cyan().underlined()
    ))?;

    Ok(())
}
