fn main() {
    let mut hand1 = ["1", "2", "3", "4"].to_vec();
    let mut counter1 = 0;
    for _ in 1..20 {
        (hand1, counter1) = keysmash(hand1.to_vec(), counter1);
    }
}

fn keysmash(mut keysmash: Vec<&str>, mut counter: usize) -> (Vec<&str>, usize) {
    let s = keysmash[counter];
    print!("{s}");
    counter = counter + 1;
    if counter == keysmash.len() - 1 {
        counter = 0;
        keysmash.swap(1, 3);
    }
    (keysmash, counter)
}
