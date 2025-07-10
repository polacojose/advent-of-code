use crate::region::region_layout;
use glam::IVec2;
use grid::grid::map::Grid;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
pub struct PlotNode(pub(super) char);

impl From<char> for PlotNode {
    fn from(c: char) -> Self {
        PlotNode(c)
    }
}

impl Display for PlotNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parameter_price_map(map: &Grid<PlotNode>) -> u64 {
    let mut counted_area = HashSet::new();

    let mut price = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {
            let p = IVec2 { x, y };

            if counted_area.contains(&p) {
                continue;
            }

            let mut local_counted_region = HashSet::new();
            let region_price = parameter_price_region(p, map, &mut local_counted_region);
            counted_area.extend(local_counted_region);
            price += region_price;
        }
    }

    price
}

pub fn side_price_map(map: &Grid<PlotNode>) -> u64 {
    let mut counted_area = HashSet::new();

    let mut price = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {
            let p = IVec2 { x, y };

            if counted_area.contains(&p) {
                continue;
            }

            let mut local_counted_region = HashSet::new();
            let region_price = side_price_region(p, map, &mut local_counted_region);
            counted_area.extend(local_counted_region);
            price += region_price;
        }
    }

    price
}

fn parameter_price_region(
    p: IVec2,
    grid: &Grid<PlotNode>,
    counted_area: &mut HashSet<IVec2>,
) -> u64 {
    let layout = region_layout(p, grid, counted_area);
    layout.area() * layout.parameter()
}

fn side_price_region(p: IVec2, grid: &Grid<PlotNode>, counted_area: &mut HashSet<IVec2>) -> u64 {
    let layout = region_layout(p, grid, counted_area);

    layout.area() * layout.sides()
}

#[cfg(test)]
mod test {
    use crate::{region::RegionLayout, util::sides_from_angles};

    use super::*;

    const TEST_PLOTS_1: &str = r"AAAA
                                 BBCD
                                 BBCC
                                 EEEC";

    const TEST_PLOTS_2: &str = r"OOOOO
                                 OXOXO
                                 OOOOO
                                 OXOXO
                                 OOOOO";

    const TEST_PLOTS_3: &str = r"RRRRIICCFF
                                 RRRRIICCCF
                                 VVRRRCCFFF
                                 VVRCCCJFFF
                                 VVVVCJJCFE
                                 VVIVCCJJEE
                                 VVIIICJJEE
                                 MIIIIIJJEE
                                 MIIISIJEEE
                                 MMMISSJEEE";

    const TEST_PLOTS_4: &str = r"EEEEE
                                 EXXXX
                                 EEEEE
                                 EXXXX
                                 EEEEE";

    const TEST_PLOTS_5: &str = r"AAAAAA
                                 AAABBA
                                 AAABBA
                                 ABBAAA
                                 ABBAAA
                                 AAAAAA";

    #[test]
    fn test_region_layout() {
        let plot_grid = TEST_PLOTS_1.parse::<Grid<PlotNode>>().unwrap();
        let layout = region_layout(IVec2 { x: 0, y: 0 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(4, 10, 360));

        let layout = region_layout(IVec2 { x: 0, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(4, 8, 360));

        let layout = region_layout(IVec2 { x: 2, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(4, 10, 720));

        let layout = region_layout(IVec2 { x: 3, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(1, 4, 360));

        let layout = region_layout(IVec2 { x: 0, y: 3 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(3, 8, 360));
    }

    #[test]
    fn test_region_layout_2() {
        let plot_grid = TEST_PLOTS_2.parse::<Grid<PlotNode>>().unwrap();
        let layout = region_layout(IVec2 { x: 1, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(1, 4, 360));

        let layout = region_layout(IVec2 { x: 3, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(1, 4, 360));

        let layout = region_layout(IVec2 { x: 1, y: 3 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(1, 4, 360));

        let layout = region_layout(IVec2 { x: 3, y: 3 }, &plot_grid, &mut HashSet::new());
        assert_eq!(layout, RegionLayout::new(1, 4, 360));

        let layout = region_layout(IVec2 { x: 0, y: 0 }, &plot_grid, &mut HashSet::new());
        println!("Sides: {}", sides_from_angles(1800));
        assert_eq!(layout, RegionLayout::new(21, 36, 1800));
    }

    #[test]
    fn test_parameter_price_region() {
        let plot_grid = TEST_PLOTS_1.parse::<Grid<PlotNode>>().unwrap();
        let price = parameter_price_region(IVec2 { x: 0, y: 0 }, &plot_grid, &mut HashSet::new());
        assert_eq!(price, 40);

        let price = parameter_price_region(IVec2 { x: 0, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(price, 32);

        let price = parameter_price_region(IVec2 { x: 2, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(price, 40);

        let price = parameter_price_region(IVec2 { x: 3, y: 1 }, &plot_grid, &mut HashSet::new());
        assert_eq!(price, 4);

        let price = parameter_price_region(IVec2 { x: 0, y: 3 }, &plot_grid, &mut HashSet::new());
        assert_eq!(price, 24);
    }

    #[test]
    fn test_sides_per_region() {
        let plot_grid = TEST_PLOTS_5.parse::<Grid<PlotNode>>().unwrap();
        let layout = region_layout(IVec2 { x: 3, y: 1 }, &plot_grid, &mut HashSet::new());
        println!("Sides: {}", sides_from_angles(1080));
        println!("layout: {:?}", layout);
        assert_eq!(layout, RegionLayout::new(21, 36, 1080));
        let price = side_price_map(&plot_grid);
        assert_eq!(price, 368);
    }

    #[test]
    fn test_price_map() {
        let plot_grid = TEST_PLOTS_1.parse::<Grid<PlotNode>>().unwrap();
        let price = parameter_price_map(&plot_grid);
        assert_eq!(price, 140);

        let plot_grid = TEST_PLOTS_1.parse::<Grid<PlotNode>>().unwrap();
        let price = side_price_map(&plot_grid);
        assert_eq!(price, 80);

        let plot_grid = TEST_PLOTS_3.parse::<Grid<PlotNode>>().unwrap();
        let price = parameter_price_map(&plot_grid);
        assert_eq!(price, 1930);

        let plot_grid = TEST_PLOTS_3.parse::<Grid<PlotNode>>().unwrap();
        let price = side_price_map(&plot_grid);
        assert_eq!(price, 1206);

        let plot_grid = TEST_PLOTS_4.parse::<Grid<PlotNode>>().unwrap();
        let price = side_price_map(&plot_grid);
        assert_eq!(price, 236);

        let plot_grid = TEST_PLOTS_5.parse::<Grid<PlotNode>>().unwrap();
        let price = side_price_map(&plot_grid);
        assert_eq!(price, 368);
    }
}
