use std::fs;

const GRID_X: isize = 100;
const GRID_Y: isize = 100;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut light_array = Vec::new();
    for line in file_string.lines() {
        let mut light_line = Vec::new();

        for byte in line.bytes() {
            match byte {
                b'#' => light_line.push(true),
                b'.' => light_line.push(false),
                _ => panic!("Invalid input"),
            }
        }

        light_array.push(light_line);
    }

    print_light_array(&light_array);
    println!();

    for _ in 0..100 {
        light_array = step_lights(light_array);
        print_light_array(&light_array);
        println!();
    }

    println!(
        "Light on {}",
        light_array.iter().flatten().filter(|x| **x).count(),
    );
}

fn print_light_array(light_array: &Vec<Vec<bool>>) {
    for line in light_array {
        for light in line {
            print!("{}", if *light { '#' } else { '.' });
        }
        println!();
    }
}

fn step_lights(light_array: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut buf_light_array = Vec::new();

    for (y, line) in light_array.iter().enumerate() {
        let mut buff_light_line = Vec::new();
        for (x, light) in line.iter().enumerate() {
            buff_light_line.push(update_light(&light_array, x, y, *light));
        }
        buf_light_array.push(buff_light_line);
    }
    buf_light_array
}

fn update_light(light_array: &Vec<Vec<bool>>, x: usize, y: usize, light: bool) -> bool {
    let neightbors_on = get_neighbors_on(x, y, &light_array);
    match light {
        true => {
            if neightbors_on == 2 || neightbors_on == 3 {
                true
            } else {
                false
            }
        }
        false => {
            if neightbors_on == 3 {
                true
            } else {
                false
            }
        }
    }
}

fn get_neighbors_on(x: usize, y: usize, light_array: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for in_y in (y as isize - 1)..=(y as isize + 1) {
        for in_x in (x as isize - 1)..=(x as isize + 1) {
            if x as isize == in_x && y as isize == in_y {
                continue;
            }

            if in_x < 0 || in_y < 0 || in_x >= GRID_X || in_y >= GRID_Y {
                continue;
            }

            let on_num = match light_array[in_y as usize][in_x as usize] {
                true => 1,
                false => 0,
            };

            count += on_num;
        }
    }

    count
}
