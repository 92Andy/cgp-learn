
fn group_letter(name: &str) -> &str {
    
    match name {
        "Plantex" => "C",
        "AVZ-Run" => "A",
        "Space-Game" => "B",
        _ => "None",
    }
}

fn main() {
    println!("{}", group_letter("Plantex"));
}

