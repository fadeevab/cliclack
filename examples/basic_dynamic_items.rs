use console::style;

fn main() -> std::io::Result<()> {
    // Set a no-op Ctrl-C handler so that Ctrl-C results in a
    // `term.read_key()` error instead of terminating the process. You can skip
    // this step if you have your own Ctrl-C handler already set up.
    //
    // We cannot (easily) handle this at the library level due to
    // https://github.com/Detegr/rust-ctrlc/issues/106#issuecomment-1887793468.
    ctrlc::set_handler(move || {}).expect("Error setting Ctrl-C handler");

    cliclack::clear_screen()?;

    cliclack::intro(style(" create-app ").on_cyan().black())?;

    // This is the only difference between this snippet and examples/basic.rs
    // You can supply your items dynamically, i.e. from a database or API.
    let items_for_select = vec![
        ("ts", "TypeScript", ""),
        ("js", "JavaScript", ""),
        ("coffee", "CoffeeScript", "oh no"),
    ];

    let _selected_dynamic_item = cliclack::select("Pick a project type")
        .initial_value("ts")
        .items(&items_for_select)
        .interact()?;

    let items_for_multiselect = &[
        ("prettier", "Prettier", "recommended"),
        ("eslint", "ESLint", "recommended"),
        ("stylelint", "Stylelint", ""),
        ("gh-action", "GitHub Action", ""),
    ];

    let _tools = cliclack::multiselect("Select additional tools")
        .initial_values(vec!["prettier", "eslint"])
        .items(items_for_multiselect)
        .interact()?;

    cliclack::outro(format!(
        "Problems? {}\n",
        style("https://example.com/issues").cyan().underlined()
    ))?;

    Ok(())
}
