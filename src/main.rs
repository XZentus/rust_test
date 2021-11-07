extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Clone)]
struct MatrixCreatorElement {
    value: u32,
    remaining_values: Vec<u32>,
    number_nonzero: u32,
} 

fn gen_matrix(size: usize, rng: &mut ThreadRng) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![MatrixCreatorElement{value: 0,
                                                    remaining_values: (1..=size as u32).collect(),
                                                    number_nonzero: size as u32};
                               size];
                          size];
    struct Env<'a> { matrix: &'a mut Vec<Vec<MatrixCreatorElement>>,
                     size: usize,
                     rng: &'a mut ThreadRng}
    fn gen_next_value(x: usize, y: usize, env: &mut Env) -> bool { // true => success, false => error
        if y >= env.size { // end of generating
            return true;
        } else if x >= env.size { // end of row
            return gen_next_value(0, y + 1, env);
        }

        loop {
            println!("{} {} {:?}", x, y, env.matrix[y][x].remaining_values);
            let number_nonzero = env.matrix[y][x].number_nonzero;
            if number_nonzero == 0 {
                return false;
            }
            let new_val = if number_nonzero == 1 {
                              let mut val = 0;
                              for i in env.matrix[y][x].remaining_values.iter() {
                                  if *i != 0 {
                                      val = *i;
                                      break;
                                  }
                              }
                              val
                          } else {
                              let mut i = env.rng.gen_range(0..number_nonzero as u32);
                              let mut res = 0;
                              // get i-th number in remaining values
                              for n in env.matrix[y][x].remaining_values.iter() {
                                  if *n != 0 {
                                      if i == 0 {
                                          res = *n;
                                          break;
                                      }
                                      else {
                                          i -= 1;
                                      }
                                  }
                              }
                              res
                          };
            env.matrix[y][x].value = new_val;
            println!("{}", new_val);
            env.matrix[y][x].remaining_values[new_val as usize - 1] = 0;
            env.matrix[y][x].number_nonzero -= 1;

            for x in x+1..env.size {
                env.matrix[y][x].remaining_values[new_val as usize - 1] = 0;
                if env.matrix[y][x].number_nonzero == 0 {
                    return false
                } else {
                    env.matrix[y][x].number_nonzero -= 1;
                }
            }
            for y in y+1..env.size {
                env.matrix[y][x].remaining_values[new_val as usize - 1] = 0;
                if env.matrix[y][x].number_nonzero == 0 {
                    return false
                } else {
                    env.matrix[y][x].number_nonzero -= 1;
                }
            }
            if gen_next_value(x + 1, y, env) {
                return true;
            }
        }
    }

    gen_next_value(0, 0, &mut Env{ matrix: &mut matrix,
                                   size: size,
                                   rng: rng});

    matrix.into_iter()
          .map(|row| row.into_iter()
                        .map(|e| e.value)
                        .collect())
          .collect()
}

fn main() {
    let mut rng = thread_rng();
    let init_line: Vec<Vec<u32>> = gen_matrix(3, &mut rng);
    for row in init_line.iter() {
        println!("{:?}", row);
    }
}
