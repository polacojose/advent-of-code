use std::{str::FromStr, sync::RwLock};

use rustler::{Env, ResourceArc, Term};

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub struct Grid {
    content: RwLock<Vec<Vec<bool>>>,
}

#[derive(Debug)]
pub struct UnableToParseGrid;
impl FromStr for Grid {
    type Err = UnableToParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|on_char| match on_char {
                        '#' => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();

        Ok(Grid {
            content: RwLock::new(content),
        })
    }
}

fn load(env: Env, _term: Term) -> bool {
    rustler::resource!(Grid, env);
    true
}

#[rustler::nif]
fn parse_grid(str: String) -> ResourceArc<Grid> {
    ResourceArc::new(str.parse().unwrap())
}

#[rustler::nif]
fn step_grid(grid: ResourceArc<Grid>) -> ResourceArc<Grid> {
    let old_grid_content: Vec<Vec<bool>> = (*grid).content.read().unwrap().to_vec();

    let mut new_grid_content = Vec::new();
    for y in 0..old_grid_content.len() {
        let mut line = Vec::new();
        for x in 0..old_grid_content[y].len() {
            let on = old_grid_content[y][x];
            let neighbors_on = neighbors_on(grid.clone(), (x, y));
            let new_on = if on {
                match neighbors_on {
                    2 | 3 => true,
                    _ => false,
                }
            } else {
                match neighbors_on {
                    3 => true,
                    _ => false,
                }
            };
            line.push(new_on);
        }
        new_grid_content.push(line);
    }

    return ResourceArc::new(Grid {
        content: RwLock::new(new_grid_content),
    });
}

fn neighbors_on(grid: ResourceArc<Grid>, loc: (usize, usize)) -> u64 {
    let content: Vec<Vec<bool>> = (*grid).content.read().unwrap().to_vec();
    let mut count = 0;
    for y in loc.1 as isize - 1..=loc.1 as isize + 1 {
        if y < 0 || y as usize >= content.len() {
            continue;
        }
        for x in loc.0 as isize - 1..=loc.0 as isize + 1 {
            if x < 0 || x as usize >= content[y as usize].len() {
                continue;
            }
            if (x as usize, y as usize) == loc {
                continue;
            }
            if content[y as usize][x as usize] {
                count += 1;
            }
        }
    }
    return count;
}

#[rustler::nif]
fn stick_corners_on(grid: ResourceArc<Grid>) -> ResourceArc<Grid> {
    {
        let mut content = (*grid).content.write().unwrap();
        let last_index_y = content.len() - 1;
        let last_index_x = content[0].len() - 1;
        content[0][0] = true;
        content[0][last_index_y] = true;
        content[last_index_x][0] = true;
        content[last_index_x][last_index_y] = true;
    }
    return grid;
}

#[rustler::nif]
fn lights_on(grid: ResourceArc<Grid>) -> u64 {
    let content = (*grid).content.read().unwrap();
    content
        .iter()
        .map(|y| y.iter().map(|x| if *x { 1 } else { 0 }).sum::<u64>())
        .sum()
}

#[rustler::nif]
fn output(grid: ResourceArc<Grid>) -> String {
    let content = (*grid).content.read().unwrap();
    let mut str_bytes = Vec::new();
    for y in content.iter() {
        for x in y {
            str_bytes.push(if *x { '#' } else { '.' })
        }
        str_bytes.push('\n');
    }
    str_bytes.into_iter().collect()
}

rustler::init!(
    "Elixir.Day18",
    [parse_grid, step_grid, stick_corners_on, lights_on, output],
    load = load
);
