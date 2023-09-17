use rand::{thread_rng, Rng};

fn main() {
    keysmash();
}

fn keysmash() {
    let mut keysmash = [1, 2, 3, 4].to_vec();
    let mut cycles = 30;
    let mut rng = thread_rng();
    while cycles != 0 {
        let a: usize = rng.gen_range(0..4);
        let b: usize = rng.gen_range(0..4);
        let s = keysmash.iter().map(|n| n.to_string()).collect::<String>();
        print!("{s}");
        cycles = cycles - 1;
        keysmash.swap(a, b);
        }
    println!("");
}
