

fn group_letter(name: &str) -> Option<char> {
    
    match name {
        "Plantex" => Some('C'),
        "AVZ-Run" => Some('A'),
        "Space-Game" => Some('B'),
        _ => None,
    }
}

fn main() {
    println!("{:?}", group_letter("Plantex"));
}

