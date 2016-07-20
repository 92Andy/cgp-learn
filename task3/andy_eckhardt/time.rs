
fn get_message(time: u8) -> String {
       
    
    match time {
        6...7 | 13...17 | 23...24 => format!("Hallo!"),
        8...12 => format!("Guten Morgen"),
        0...5 => format!("Warum bist du denn um {} noch wach ?", time),
        18...22 => format!("Guten Abend"),
        _ => format!("Fehler"),
    }
}

fn main() {
    println!("{}", getMessage(4));
}


