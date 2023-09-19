use clap::Parser;
use rand::{thread_rng, Rng};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;
/// Generates cromulent keysmash
#[derive(Parser, Debug)]
#[clap(version, about)]
#[command(name = "keysmashgen")]
#[command(version = "0.1.0")]
struct Args {
    /// number of keypresses per hand
    #[clap(short, long, default_value_t = 50)]
    length: usize,
    /// probability of mutation every cycle of keysmash
    #[clap(short, long, default_value_t = 1.0)]
    probability: f32,
    /// keys that each finger of a hand is on. May be specified multiple times for multiple hands
    #[clap(required = true)]
    hands: Vec<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let length: usize = args.length;
    let probability: f32 = args.probability;

    match probability {
        p if !(0.0..=1.0).contains(&p) => {
            println!("probability must be between 0 to 1 (inclusive)");
            return ExitCode::from(1);
        }
        _ => (),
    }

    let mut children = Vec::new();
    for hand in args.hands {
        children.push(thread::spawn(move || {
            let handlen = hand.len();
            let mut counter = handlen;
            let mut rng = thread_rng();
            for _ in 0..length {
                let mut hand: Vec<char> = hand.chars().collect::<Vec<char>>();
                if counter == handlen {
                    counter = 0;
                    if rng.gen::<f32>() < probability {
                        hand.swap(rng.gen_range(0..handlen), rng.gen_range(0..handlen));
                    }
                }
                counter = keysmash(hand.clone(), counter);
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }
    for child in children {
        child.join().unwrap();
    }
    ExitCode::from(0)
}

fn keysmash(keysmash: Vec<char>, counter: usize) -> usize {
    let s = keysmash[counter];
    print!("{s}");
    counter + 1
}
