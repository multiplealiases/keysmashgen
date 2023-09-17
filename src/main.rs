use std::thread;
use std::time::Duration;

fn main() {
    let hands = hands.as_slice();
    let mut children = vec![];
    for t in 0..hands.len() {
        println!("spawning thread {t}");
        children.push(thread::spawn(move || {
            let mut hand: Vec<&str> = hands[t].to_vec();
            let mut counter = 0;
            for _ in 0..50 {
                (hand, counter) = keysmash(hand, counter);
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    for child in children {
        child.join().unwrap();
    }
}

fn keysmash(mut keysmash: Vec<&str>, mut counter: usize) -> (Vec<&str>, usize) {
    let s = keysmash[counter];
    print!("{s}");
    counter = counter + 1;
    if counter == keysmash.len() {
        counter = 0;
        keysmash.swap(1, 3);
    }
    (keysmash, counter)
}
