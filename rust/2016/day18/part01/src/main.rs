fn map_floor_traps(first_line: impl Into<String>, height: usize) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = vec![first_line
        .into()
        .chars()
        .map(|x| match x {
            '^' => true,
            _ => false,
        })
        .collect()];

    let map_width = map.first().unwrap().len();

    for y in 1..height {
        map.push(Vec::new());
        for x in 0..map_width {
            let a = if x == 0 { false } else { map[y - 1][x - 1] };
            let b = map[y - 1][x];
            let c = if x == (map_width - 1) {
                false
            } else {
                map[y - 1][x + 1]
            };

            let trapped = if (a == true && b == true && c == false)
                || (a == false && b == true && c == true)
                || (a == true && b == false && c == false)
                || (a == false && b == false && c == true)
            {
                true
            } else {
                false
            };

            map[y].push(trapped);
        }
    }

    map
}

fn main() {
    let first_line = "^.^^^.^..^....^^....^^^^.^^.^...^^.^.^^.^^.^^..^.^...^.^..^.^^.^..^.....^^^.^.^^^..^^...^^^...^...^.";
    let height = 400000;
    let floor_map = map_floor_traps(first_line, height);

    let mut floor_map_string = String::new();
    for y in 0..height {
        for x in 0..first_line.len() {
            floor_map_string.push(match floor_map[y][x] {
                true => '^',
                false => '.',
            });
        }
        floor_map_string.push('\n');
    }
    //println!("{}", floor_map_string);

    println!(
        "Safe tiles: {}",
        floor_map
            .iter()
            .flatten()
            .filter(|trapped| !**trapped)
            .count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping() {
        let first_line = ".^^.^.^^^^";
        let height = 10;
        let floor_map = map_floor_traps(first_line, height);
        println!("{:?}", floor_map);

        let mut floor_map_string = String::new();
        for y in 0..height {
            for x in 0..first_line.len() {
                floor_map_string.push(match floor_map[y][x] {
                    true => '^',
                    false => '.',
                });
            }
            floor_map_string.push('\n');
        }
        println!("{}", floor_map_string);

        println!(
            "Safe tiles: {}",
            floor_map
                .iter()
                .flatten()
                .filter(|trapped| !**trapped)
                .count()
        );
    }
}
