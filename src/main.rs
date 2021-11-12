extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::rngs::ThreadRng;

fn gen_matrix(size: usize, rng: &mut ThreadRng) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size]; size];
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

            if env.m[y][x] == 0 {
                println!("ZERO: {:?}, {} {}", buffer, x, y);
            }

            if gen_next_value(x + 1, y, env) { return true; }
        }
    }

    gen_next_value(0, 0, &mut Env{ m: &mut matrix,
                                   size: size,
                                   rng: rng });
    matrix
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
    let init_line: Vec<Vec<u32>> = gen_matrix(19, &mut rng);
    for row in init_line.iter() {
        println!("{:?}", row);
    }
}
