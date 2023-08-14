use std::{
    fs,
    ops::{Add, AddAssign},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Vector(isize, isize, isize);

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

#[derive(Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Particle {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([-0-9]+)").unwrap();

        let capture = re
            .find_iter(s)
            .map(|x| x.as_str().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let mut capture_chunks = capture.chunks(3);

        let mut position = Vector(0, 0, 0);
        if let Some(p) = capture_chunks.next() {
            position.0 = p[0];
            position.1 = p[1];
            position.2 = p[2];
        }
        let mut velocity = Vector(0, 0, 0);
        if let Some(p) = capture_chunks.next() {
            velocity.0 = p[0];
            velocity.1 = p[1];
            velocity.2 = p[2];
        }
        let mut acceleration = Vector(0, 0, 0);
        if let Some(p) = capture_chunks.next() {
            acceleration.0 = p[0];
            acceleration.1 = p[1];
            acceleration.2 = p[2];
        }

        Ok(Particle {
            position,
            velocity,
            acceleration,
        })
    }
}

fn get_distance_origin(particle: &Particle) -> usize {
    (particle.position.0.abs() + particle.position.1.abs() + particle.position.2.abs()) as usize
}

fn closest_origin_particle(particles: &Vec<Particle>) -> &Particle {
    let mut min_distance = usize::MAX;
    let mut min_particle = particles.get(0).unwrap();

    for p in particles {
        let distance = get_distance_origin(p);
        if distance < min_distance {
            min_distance = distance;
            min_particle = p;
        }
    }

    min_particle
}

fn update_particles(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        p.velocity += p.acceleration;
    }
    for p in particles.iter_mut() {
        p.position += p.velocity;
    }
}

fn main() {
    collide();
}

fn collide() {
    let mut particles = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Particle>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..100000 {
        update_particles(&mut particles);
        particles.sort_unstable_by_key(|item| (item.position.0, item.position.1, item.position.2));
        particles.dedup_by_key(|item| (item.position.0, item.position.1, item.position.2));
    }

    let mut particles = particles.into_iter().enumerate().collect::<Vec<_>>();
    particles.sort_by(|(_, a), (_, b)| get_distance_origin(a).cmp(&get_distance_origin(b)));

    println!("{:#?}", particles.first());
}

fn non_collide() {
    let mut particles = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Particle>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", particles);

    for _ in 0..100000 {
        update_particles(&mut particles);
    }

    let mut particles = particles.into_iter().enumerate().collect::<Vec<_>>();
    particles.sort_by(|(_, a), (_, b)| get_distance_origin(a).cmp(&get_distance_origin(b)));

    println!("{:#?}", particles.first());
}
