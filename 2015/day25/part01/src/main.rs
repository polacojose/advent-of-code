fn main() {
    let coord = (3019, 3010);
    //let coord = (2, 3);
    let iterations = get_iteration_number(coord.0, coord.1);
    println!("({},{}) = Iterations: {}", coord.0, coord.1, iterations);

    let mut code: u64 = 20151125;
    for _ in 1..iterations {
        code *= 252533;
        code %= 33554393;
    }

    println!("Code: {}", code);
}

fn get_iteration_number(x: usize, y: usize) -> usize {
    if x == 1 && y == 1 {
        return 1;
    }

    if x == 1 {
        return (y - 1) + get_iteration_number(1, y - 1);
    }

    if y == 1 {
        return x + get_iteration_number(x - 1, y);
    }

    (x - 1) + y + get_iteration_number(x - 1, y)
}
