use std::fs;
use std::io;

const DIRS: [(i32,i32); 4] = [
    (0,1), //NORTH
    (1,0), //EAST
    (0,-1), //SOUTH
    (-1,0), //WEST
];

struct Information{
   horizontal_distance: i32,
   vertical_distance: i32,
   current_dir: i32,
   visited: Vec<(i32,i32)>,
}

fn main() {
    // let args:Vec<String> = env::args().collect();
    // let file_name: &str = &args[1];
    let content:String = read_file("input.txt").expect("Error reading file.");
    println!("The distance to Easter Bunny Headquarters is: {}", distance_to_ebhq(&content));
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("inputs/{file_name}"))
}

fn distance_to_ebhq(steps: &String) -> i32 {
    let steps_iter = &steps.replace(','," ");
    let steps_iter = steps_iter.split_whitespace();

    let mut info = Information {
        current_dir: 0,
        horizontal_distance: 0,
        vertical_distance: 0,
        visited: Vec::new()
    };

    for i in steps_iter{
        let dir = &i[0..1];
        let n_steps: i32 = (&i[1..]).trim().parse().expect("Error parsing to number.");
        if next_move(&mut info, dir, n_steps){ break; }
    }
    info.horizontal_distance.abs() + info.vertical_distance.abs()
}

fn next_move(info: &mut Information, dir: &str, n_steps: i32)-> bool {
    info.current_dir = if dir == "R" {
        (info.current_dir + 1) % DIRS.len() as i32
    } else {
        (info.current_dir - 1).rem_euclid(DIRS.len() as i32)
    };
    for _ in 0..n_steps{
        info.horizontal_distance += DIRS[info.current_dir as usize].0;
        info.vertical_distance += DIRS[info.current_dir as usize].1;
        let is_hq = &info.visited.iter().any(|(x,y)| *x == info.horizontal_distance && *y == info.vertical_distance);
        if *is_hq {
            return true;
        }
        (&mut info.visited).push((info.horizontal_distance, info.vertical_distance));
    }
    false
}
