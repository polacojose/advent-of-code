use part01::NODES;

fn main() {
    let mut pairs = Vec::new();
    for i in 0..NODES.len() {
        let ref node_a = NODES[i];
        if node_a.used > 0 {
            for j in 0..NODES.len() {
                let ref node_b = NODES[j];
                if node_a == node_b {
                    continue;
                }
                if node_a.used <= node_b.avail {
                    let pair = (i, j);
                    pairs.push(pair);
                }
            }
        }
    }
    println!("{}", pairs.len());
}
