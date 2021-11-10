extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::rngs::ThreadRng;

fn gen_matrix(size: usize, rng: &mut ThreadRng) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size]];
    struct Env<'a> { m: &'a mut Vec<Vec<u32>>,
                     size: usize,
                     rng: &'a mut ThreadRng };
    fn gen_next_value(x: usize, y: usize, env: &mut Env) -> bool { // true => success, false => error
        if y > env.size { return true; }
        if x > env.size { return get_next_value(0, y + 1, env); }
        loop {
            ;
        }
    }

    gen_next_value(0, 0, &mut Env{ m: &mut matrix,
                                   size: size,
                                   rng: rng});
    matrix
}

fn get_allowed_nums(x: usize, y: usize, m: &Vec<Vec<u32>>, res: &mut Vec<u32>) {
    let size = m.len();
    res.resize(size, 0);
    for i in 1..=size {
        res[i - 1] = i as u32;
    }
    for _x in 0..size {
        if _x == x { continue; }
        res[(m[y][_x] - 1) as usize] = 0;
    }
    for _y in 0..size {
        if _y == y { continue; }
        res[(m[_y][x] - 1) as usize] = 0;
    }
    res.retain(|&x| x != 0);
}

fn main() {
    let mut rng = thread_rng();
    let init_line: Vec<Vec<u32>> = gen_matrix(3, &mut rng);
    for row in init_line.iter() {
        println!("{:?}", row);
    }
}
