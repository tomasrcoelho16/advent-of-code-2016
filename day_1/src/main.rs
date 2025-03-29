use std::env;
use std::fs;

struct Information{
   horizontal_distance: i32,
   vertical_distance: i32,
   is_horizontal: bool,
   is_positive: bool,
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

fn distance_to_ebhq(steps: &String) -> i32 {
    let steps_iter = &steps.replace(','," ");
    let steps_iter = steps_iter.split_whitespace();
    let mut info = Information {
        is_horizontal: false,
        is_positive: true,
        horizontal_distance: 0,
        vertical_distance: 0,
    };
    for i in steps_iter{
        let dir = &i[0..1];
        let n_steps: i32 = (&i[1..]).trim().parse().expect("Error parsing to number.");
        next_move(&mut info, dir, n_steps);
    }
    info.horizontal_distance.abs() + info.vertical_distance.abs()
}

fn next_move(info: &mut Information, dir: &str, n_steps: i32){
    info.is_horizontal = !info.is_horizontal;
    if info.is_horizontal {
        if info.is_positive {
            info.horizontal_distance += if dir == "R" {
                n_steps
            } else {
                info.is_positive = !info.is_positive;
                -n_steps
            };
        } else {
            info.horizontal_distance += if dir == "R" {
                -n_steps
            } else {
                info.is_positive = !info.is_positive;
                n_steps
            };
        }
        return
    }

    if info.is_positive {
        info.horizontal_distance += if dir == "R" {
            info.is_positive = !info.is_positive;
            n_steps
        } else {
            -n_steps
        };
    } else {
        info.horizontal_distance += if dir == "R" {
            info.is_positive = !info.is_positive;
            -n_steps
        } else {
            n_steps
        };
    }
}
