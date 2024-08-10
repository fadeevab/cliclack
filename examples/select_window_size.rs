fn main() -> std::io::Result<()> {
    let mut items: Vec<(String, String, String)> = Vec::new();

    for i in 0..20 {
        items.push((
            format!("Item {}", i),
            i.to_string(),
            format!("Hint {}", i),
        ));
    }

    let selected = cliclack::select("Select an item")
        .items(&items)
        .window_size(5)
        .filter_mode() // Try filtering on "1"
        .interact()?;

    cliclack::outro(format!("You selected: {}", selected))?;

    Ok(())
}