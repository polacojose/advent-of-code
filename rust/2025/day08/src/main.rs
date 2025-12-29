use std::fs;

use crate::connection::{Connection, ConnectionStore, Pos};

pub mod connection;

fn main() {
    let vec_pos: Vec<Pos> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("Unable to parse");

    let dists = get_dist_sorted_connections(&vec_pos);

    let mut store = ConnectionStore::default();
    for i in 0..(dists.len().min(1000)) {
        let conn = &dists[i];
        store.insert(conn.a, conn.b);
    }

    let r = &store
        .get_connections_by_size()
        .iter()
        .rev()
        .collect::<Vec<_>>()[..3]
        .iter()
        .fold(1, |acc, a| acc * a.len());

    println!("Part 1: {r}");

    let mut store = ConnectionStore::default();
    let (a, b) = {
        let mut result = None;
        for i in 0..dists.len() {
            let conn = &dists[i];
            store.insert(conn.a, conn.b);

            if store.num_connected() == vec_pos.len() {
                println!("Inserted: {}, {})", conn.a, conn.b,);
                result = Some((conn.a, conn.b));
                break;
            }
        }

        result
    }
    .unwrap();

    println!("Part 2: {}", a.x * b.x);
}

fn get_dist_sorted_connections(vec_pos: &Vec<Pos>) -> Vec<Connection> {
    let mut dists = vec![];
    for i in 0..vec_pos.len() {
        let a = vec_pos[i];
        for j in i + 1..vec_pos.len() {
            let b = vec_pos[j];
            dists.push(Connection::new(a, b));
        }
    }

    dists.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    dists
}

#[cfg(test)]
mod tests {
    use crate::connection::{Connection, ConnectionStore, Pos};

    const TEST_DATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_dist() {
        assert_eq!(Pos::new(0, 0, 0).dist(&Pos::new(0, 0, 0)).round(), 0.0);
        assert_eq!(Pos::new(0, 0, 0).dist(&Pos::new(1, 0, 0)).round(), 1.0);
        assert_eq!(Pos::new(0, 0, 0).dist(&Pos::new(0, 1, 0)).round(), 1.0);
        assert_eq!(Pos::new(0, 0, 0).dist(&Pos::new(0, 0, 1)).round(), 1.0);
    }

    #[test]
    fn test_parse() {
        let vec_pos: Vec<Pos> = TEST_DATA
            .trim()
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("Unable to parse");

        let mut dists = vec![];
        for i in 0..vec_pos.len() {
            let a = vec_pos[i];
            for j in i + 1..vec_pos.len() {
                let b = vec_pos[j];
                dists.push(Connection::new(a, b));
            }
        }

        dists.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        let mut store = ConnectionStore::default();
        for i in 0..(dists.len().min(10)) {
            let conn = &dists[i];
            store.insert(conn.a, conn.b);
            let new_id = store.id_by_pos(&conn.a).unwrap();
            println!(
                "Inserted: {}, {} to {}({})",
                conn.a,
                conn.b,
                new_id,
                store.get_connection(new_id).unwrap().iter().len()
            );
        }

        let r = &store
            .get_connections_by_size()
            .iter()
            .rev()
            .collect::<Vec<_>>()[..3]
            .iter()
            .fold(1, |acc, a| acc * a.len());

        assert_eq!(*r, 40)
    }

    #[test]
    fn test_big_circuit() {
        let vec_pos: Vec<Pos> = TEST_DATA
            .trim()
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("Unable to parse");

        let mut dists = vec![];
        for i in 0..vec_pos.len() {
            let a = vec_pos[i];
            for j in i + 1..vec_pos.len() {
                let b = vec_pos[j];
                dists.push(Connection::new(a, b));
            }
        }

        dists.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        let mut store = ConnectionStore::default();

        let (a, b) = {
            let mut result = None;
            for i in 0..dists.len() {
                let conn = &dists[i];
                store.insert(conn.a, conn.b);

                if store.num_connected() == vec_pos.len() {
                    println!("Inserted: {}, {})", conn.a, conn.b,);
                    result = Some((conn.a, conn.b));
                    break;
                }
            }
            result
        }
        .unwrap();

        assert_eq!(a.x * b.x, 25272)
    }
}
