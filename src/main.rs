extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = thread_rng();
    let mut init_line: Vec<u32> = (1..10).collect();
    init_line.shuffle(&mut rng);
    println!("{:?}", init_line);
}
