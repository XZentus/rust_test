extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::rngs::ThreadRng;
// use std::cmp::max;

fn gen_matrix(size: usize, rng: &mut ThreadRng) -> Vec<Vec<u32>> {
    struct Env<'a> { m: &'a mut Vec<Vec<u32>>,
                     size: usize,
                     rng: &'a mut ThreadRng }

    fn gen_next_value(x: usize, y: usize, env: &mut Env) -> bool { // true => success, false => error
        if y >= env.size { return true; }
        if x >= env.size { return gen_next_value(0, y + 1, env); }
        let mut buffer = Vec::with_capacity(env.size);
        buffer.resize(env.size, 0);
        get_allowed_nums(x, y, env.m, &mut buffer);
        loop {
            match buffer.len() {
                0 => return false,
                1 => {  env.m[y][x] = buffer[0];
                        buffer.pop();
                     },
                i => { let i = env.rng.gen_range(0..i);
                       env.m[y][x] = buffer[i];
                       buffer.swap_remove(i);
                     }
            }

            if gen_next_value(x + 1, y, env) { return true; }
        }
    }

    let mut matrix = vec![vec![0u32; size]; size];
    while !gen_next_value(0, 0, &mut Env{ m: &mut matrix,
                                         size: size,
                                         rng: rng }) {
        println!("Failed:");
        for row in matrix.iter() {
            println!("{:?}", row);
        }
        for row in &mut matrix { row.fill(0); }
    }
    matrix
}

fn print_board(m: &Vec<Vec<u32>>) {
    // let mut max_value: u32 = *m.iter()
    //                            .flatten()
    //                            .reduce(max)
    //                            .unwrap();
    let mut max_value = m.len();
    let w = { let mut w = 1;
              while max_value > 9 {
                  max_value /= 10;
                  w += 1;
              }
              w
            };
    fn print_w(x: u32, w: usize) {
        let num_s = format!("{}", x);
        print!("| ");
        (0..w - num_s.len()).for_each(|_| print!(" "));
        print!("{} ", num_s);
    }
    
    let mut sep_string = String::with_capacity((w + 3) * m.len() + 2);
    let dsh_parts: String = (0..w + 2).map(|_| '-').collect();
    for _ in 0..m.len() {
        sep_string.push('+');
        sep_string.push_str(&dsh_parts);
    }
    sep_string.push('+');

    (0..m.len()).for_each(|y| {
                    println!("{}", sep_string);
                    m[y].iter().for_each(| x | print_w(*x, w));
                    println!("|");
                });
    println!("{}", sep_string);
}

fn get_allowed_nums(x: usize, y: usize, m: &Vec<Vec<u32>>, res: &mut Vec<u32>) {
    let size = m.len();
    for i in 1..=size {
        res[i - 1] = i as u32;
    }
    for _x in 0..size {
        if _x == x || m[y][_x] == 0 { continue; }
        res[(m[y][_x] - 1) as usize] = 0;
    }
    for _y in 0..size {
        if _y == y || m[_y][x] == 0 { continue; }
        res[(m[_y][x] - 1) as usize] = 0;
    }
    res.retain(|&x| x != 0);
}

fn main() {
    let mut rng = thread_rng();
    let board: Vec<Vec<u32>> = gen_matrix(5, &mut rng);
    print_board(&board);
}
