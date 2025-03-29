use std::env;
use std::fs;

enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let file_name: &str = &args[1];
    let content:String = read_file(file_name);
    println!("The distance to Easter Bunny Headquarters is: {}", distance_to_ebhq(&content));
}

fn read_file(file_name: &str) -> String {

    println!("Reading file: {file_name}");

    fs::read_to_string(format!("inputs/{file_name}")).expect(&format!("Error finding file {file_name}."))
}

fn distance_to_ebhq(steps: &String) -> u32 {
    let steps_iter = &steps.replace(','," ");
    let steps_iter = steps_iter.split_whitespace();
    let mut current_dir:Direction = Direction::North;
    let mut x_dist: u32 = 0;
    let mut y_dist: u32 = 0;
    for i in steps_iter{
        let dir = &i[0..1];
        let n_steps: u32 = (&i[1..]).trim().parse().expect("Error parsing to number.");
    }
    x_dist + y_dist
}
