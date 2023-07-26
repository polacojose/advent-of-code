use part01::{parse_light_file, step_lights};

fn main() {
    let mut light_array = parse_light_file();
    let mut working_light_array = light_array.clone();

    //print_light_array(&light_array);
    //println!();

    for _ in 0..100 {
        step_lights(&mut light_array, &mut working_light_array);
        //print_light_array(&light_array);
        //println!();
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
