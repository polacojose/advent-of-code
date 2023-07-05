use part01::DISCS;

fn main() {
    for i in 0.. {
        if DISCS
            .iter()
            .map(|d| d.get_position_relative_start(i))
            .max()
            .unwrap()
            == 0
        {
            println!("{i}");
            break;
        }
    }
}
