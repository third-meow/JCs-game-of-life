use std::{thread, time};
use rand::Rng;

const ENV_HEIGHT :usize = 90;
const ENV_WIDTH :usize = 107;
const ESC :char = 27 as char;

fn display(env :&Vec<Vec<bool>>) {
    for row in env.iter() {
        for cell in row.iter() {
            if *cell {
                print!("{}[42m+{}[40m", ESC, ESC);
                //print!("+");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn count_neighbours(env: &Vec<Vec<bool>>, x :usize, y :usize) -> u64 {
    let mut neighbour_n :u64 = 0;

    if x > 0 {
        if env[y][x-1] {        //Left
            neighbour_n += 1
        }

        if y > 0 {
            if env[y-1][x-1] {  //Up-left
                neighbour_n += 1
            }
        }

        if y < (ENV_HEIGHT - 1) {
            if env[y+1][x-1] {  //Down-left
                neighbour_n += 1
            }
        }
    }

    if x < (ENV_WIDTH - 1) {
        if env[y][x+1] {        //Right
            neighbour_n += 1
        }

        if y > 0 {
            if env[y-1][x+1] {  //Up-right
                neighbour_n += 1
            }
        }
        if y < (ENV_HEIGHT - 1) {
            if env[y+1][x+1] {  //Down-right
                neighbour_n += 1
            }
        }
    }

    if y > 0 {
        if env[y-1][x] {        //Up
            neighbour_n += 1
        }
    }

    if y < (ENV_HEIGHT - 1) {
        if env[y+1][x] {        //Down
            neighbour_n += 1
        }
    }

    neighbour_n
}

fn take_step(env: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_env: Vec<Vec<bool>> = env.to_vec(); 
    for y in 0..env.len() { // y-axis
        for x in 0..env[y].len() { // x-axis
            let neighbour_n = count_neighbours(env, x, y);
            if neighbour_n < 2 || neighbour_n > 3 {
                // dies
                new_env[y][x] = false;
            } else if neighbour_n == 3 {
                // reproduces
                new_env[y][x] = true;
            } else {
                // stays same
                new_env[y][x] = env[y][x];
            }
        }
    }
    new_env
}


fn main() {
    let mut env: Vec<Vec<bool>> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..ENV_HEIGHT {
        env.push(Vec::new());
        for _ in 0..ENV_WIDTH {
            env[i].push(rng.gen_bool(0.24));
            //env[i].push(false);
        }
    }

    loop {
        print!("{}[2J", ESC);
        display(&env);
        thread::sleep(time::Duration::from_millis(70));
        env = take_step(&env);
    }
}
