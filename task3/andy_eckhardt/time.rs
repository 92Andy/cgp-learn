
fn getMessage(time: u8) -> String {
       
    
    match time {
        8...12 => format!("Guten Morgen"),
        0...5 => format!("Warum bist du denn um {} noch wach ?", time),
        18...22 => format!("Guten Abend"),
        _ => format!("Hallo!"),
    }
}

fn main() {
    println!("{}", getMessage(4));
}


