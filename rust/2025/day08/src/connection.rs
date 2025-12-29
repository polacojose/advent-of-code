use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    str::FromStr,
};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:3},{:3},{:3})", self.x, self.y, self.z)
    }
}

impl Pos {
    pub fn new(x: impl Into<i64>, y: impl Into<i64>, z: impl Into<i64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    pub fn dist(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

impl FromStr for Pos {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s.trim().split(",").collect();
        Ok(Self {
            x: nums[0].parse()?,
            y: nums[1].parse()?,
            z: nums[2].parse()?,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct ConnectionStore {
    pos: HashMap<Pos, String>,
    connections: HashMap<String, HashSet<Pos>>,
}

impl ConnectionStore {
    pub fn insert(&mut self, a: Pos, b: Pos) {
        let (a, b) = if self.pos.contains_key(&a) {
            (a, b)
        } else {
            (b, a)
        };

        if self.connected(a, b) {
            return;
        }

        match (self.pos.contains_key(&a), self.pos.contains_key(&b)) {
            (true, true) => self.merge_connections(a, b),
            (true, false) => {
                self.left_join_pos(a, b);
            }
            (false, false) => {
                self.gen_connection(a, b);
            }
            _ => panic!("Invalid"),
        }
    }

    fn connected(&self, a: Pos, b: Pos) -> bool {
        match (self.id_by_pos(&a), self.id_by_pos(&b)) {
            (Some(a_id), Some(b_id)) => a_id == b_id,
            _ => false,
        }
    }

    pub fn id_by_pos(&self, pos: &Pos) -> Option<&String> {
        self.pos.get(pos)
    }

    pub fn get_connection(&self, id: &str) -> Option<&HashSet<Pos>> {
        self.connections.get(id)
    }

    fn merge_connections(&mut self, a: Pos, b: Pos) {
        //Drain b
        let id = self.pos[&b].clone();
        let pos = self.connections[&id].iter().copied().collect::<Vec<_>>();
        self.connections.remove_entry(&id);

        for p in pos {
            self.left_join_pos(a, p);
        }
    }

    fn left_join_pos(&mut self, a: Pos, b: Pos) {
        let id = self.pos[&a].clone();
        self.pos.insert(b, id.clone());
        self.connections.entry(id).and_modify(|s| {
            s.insert(b);
        });
    }

    fn gen_connection(&mut self, a: Pos, b: Pos) {
        let uuid = Uuid::new_v4();
        self.pos.insert(a, uuid.to_string());
        self.pos.insert(b, uuid.to_string());
        self.connections
            .entry(uuid.to_string())
            .and_modify(|s| {
                s.insert(a);
                s.insert(b);
            })
            .or_insert({
                let mut s = HashSet::default();
                s.insert(a);
                s.insert(b);
                s
            });
    }

    pub fn num_connected(&self) -> usize {
        self.pos.len()
    }

    pub fn get_connections_by_size(&self) -> Vec<&HashSet<Pos>> {
        let mut conns = self.connections.values().collect::<Vec<_>>();
        conns.sort_by(|a, b| a.len().cmp(&b.len()));
        conns
    }
}

#[derive(Debug)]
pub struct Connection {
    pub a: Pos,
    pub b: Pos,
    pub dist: f64,
    connection_id: Option<u64>,
}

impl Connection {
    pub fn new(a: Pos, b: Pos) -> Self {
        Self {
            a,
            b,
            dist: a.dist(&b),
            connection_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_connection() {
        let mut store = ConnectionStore::default();
        let a = Pos::new(0, 0, 0);
        let b = Pos::new(1, 0, 0);
        store.insert(a, b);
        let a_id = store.id_by_pos(&a).expect("?").clone();
        let b_id = store.id_by_pos(&b).expect("?").clone();
        assert!(a_id == b_id);
    }

    #[test]
    fn test_join_connection() {
        let mut store = ConnectionStore::default();
        let a = Pos::new(0, 0, 0);
        let b = Pos::new(1, 0, 0);
        let c = Pos::new(2, 0, 0);
        store.insert(a, b);
        store.insert(a, c);
        let a_id = store.id_by_pos(&a).expect("?").clone();
        let b_id = store.id_by_pos(&b).expect("?").clone();
        let c_id = store.id_by_pos(&c).expect("?").clone();
        assert!(a_id == b_id && b_id == c_id);
    }

    #[test]
    fn test_merge_connection() {
        let mut store = ConnectionStore::default();
        let a = Pos::new(0, 0, 0);
        let b = Pos::new(1, 0, 0);
        let c = Pos::new(2, 0, 0);
        store.insert(a, b);
        store.insert(a, c);

        let a_id = store.id_by_pos(&a).expect("?").clone();
        let b_id = store.id_by_pos(&b).expect("?").clone();
        let c_id = store.id_by_pos(&c).expect("?").clone();
        assert!(a_id == b_id && b_id == c_id);

        let d = Pos::new(3, 0, 0);
        let e = Pos::new(4, 0, 0);
        let f = Pos::new(5, 0, 0);
        store.insert(d, e);
        store.insert(d, f);
        let d_id = store.id_by_pos(&d).expect("?").clone();
        let e_id = store.id_by_pos(&e).expect("?").clone();
        let f_id = store.id_by_pos(&f).expect("?").clone();
        assert!(d_id == e_id && e_id == f_id);

        store.insert(Pos::new(5, 0, 0), a);

        assert_eq!(
            [a, b, c, d, e, f]
                .into_iter()
                .filter_map(|i| store.id_by_pos(&i))
                .collect::<HashSet<_>>()
                .iter()
                .count(),
            1
        )
    }
}
