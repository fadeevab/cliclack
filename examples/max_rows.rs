fn main() -> std::io::Result<()> {
    let mut items: Vec<(String, String, String)> = Vec::new();

    for i in 0..20 {
        items.push((format!("Item {}", i), i.to_string(), format!("Hint {}", i)));
    }

    // Try this example with a terminal height both less than and greater than 10
    // to see the automatic window-size adjustment.
    let selected = cliclack::select("Select an item")
        .items(&items)
        .max_rows(10) // Specify the maximum number of rows
        .filter_mode() // Try filtering on "1"
        .interact()?;

    cliclack::outro(format!("You selected: {}", selected))?;

    Ok(())
}
