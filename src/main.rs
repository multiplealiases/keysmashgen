use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

fn main() {
    let mut children = Vec::new();
    for hand in std::env::args().skip(1) {
        children.push(thread::spawn(move || {
            let mut counter = 0;
            let mut rng = thread_rng();
            let handlen = hand.len();
            for _ in 0..50 {
                let mut hand: Vec<char> = hand.chars().collect::<Vec<char>>();
                counter = keysmash(hand.clone(), counter);
                if counter == handlen {
                    counter = 0;
                    hand.swap(rng.gen_range(0..handlen), rng.gen_range(0..handlen));
                }
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }
    for child in children {
        child.join().unwrap();
    }
}

fn keysmash(keysmash: Vec<char>, counter: usize) -> usize {
    let s = keysmash[counter];
    print!("{s}");
    counter + 1
}
