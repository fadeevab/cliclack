use cliclack::select;

fn main() {
    let selected = select("Select an anime")
        .item("hello","hello","hi")
        .item("world","world","world")
        .item("how","how","how")
        .item("are","are","are")
        .item("you","you","you")
        .item("hello you","hello you","hello you")
        .interact();

    if let Ok(val) = selected {
        println!("{}", val);
    }

}