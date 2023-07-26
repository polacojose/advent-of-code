use std::fs;

const GRID_X: usize = 100;
const GRID_Y: usize = 100;

pub fn parse_light_file() -> [[bool; GRID_X]; GRID_Y] {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut light_array = [[false; GRID_X]; GRID_Y];

    let mut lines = file_string.lines();
    for y in 0..GRID_Y {
        let line = lines.next().unwrap();
        for x in 0..GRID_X {
            for byte in line.bytes() {
                light_array[y][x] = match byte {
                    b'#' => true,
                    b'.' => false,
                    _ => panic!("Invalid input"),
                };
            }
        }
    }

    light_array
}

pub fn step_lights(
    current_light_array: &mut [[bool; GRID_X]; GRID_Y],
    working_light_array: &mut [[bool; GRID_X]; GRID_Y],
) {
    for y in 0..GRID_Y {
        for x in 0..GRID_X {
            working_light_array[y][x] = update_light(&current_light_array, x, y);
        }
    }

    for y in 0..GRID_Y {
        for x in 0..GRID_X {
            current_light_array[y][x] = working_light_array[y][x];
        }
    }
}

#[inline]
pub fn update_light(light_array: &[[bool; GRID_X]; GRID_Y], x: usize, y: usize) -> bool {
    let neightbors_on = get_neighbors_on(x, y, light_array);
    let light = light_array[y][x];
    match light {
        true => {
            //if neightbors_on == 2 || neightbors_on == 3 {
            if neightbors_on >= 2 {
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

#[inline]
pub fn get_neighbors_on(x: usize, y: usize, light_array: &[[bool; GRID_X]; GRID_Y]) -> usize {
    let mut count = 0;
    for in_y in y.saturating_sub(1) as isize..=(y as isize + 1).min(GRID_Y as isize - 1) {
        for in_x in x.saturating_sub(1) as isize..=(x as isize + 1).min(GRID_X as isize - 1) {
            if light_array[in_y as usize][in_x as usize] {
                count += 1;
            };
        }
    }

    if light_array[y][x] {
        count -= 1;
    }

    count
}
