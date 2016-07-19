
fn get_vec(n: usize) -> Vec<u32> {
    let mut v = Vec::with_capacity(n);
    let u = n as u32;
    for x in 1..u+1 {
        v.push(x);
    }
    v
}

fn get_prim(n: usize) -> Vec<u32> {
    let mut v = get_vec(n);
    let r = (n as f32).sqrt();
    for i in 2..(r as u32) {
        for j in 2..n {
            if v[j] % i == 0 && v[j] != i {
                v[j] = 0;
            }
        }
    }

    get_clear(v)
}

fn get_clear(v: Vec<u32>) -> Vec<u32> {
    let v = v;
    let mut new_v = Vec::new();

    for i in 1..v.len() {
        if v[i] != 0 {
            new_v.push(v[i]);
        }
    }
    new_v
}

fn main() {
    let n = 100;
    let v = get_prim(n);
    println!("Primzahlen bis {}", n);
    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}       
        
