use std::io::Read;

use crate::nav_reader::NavReader;

#[derive(Debug)]
pub struct NavTreeNode {
    children: Vec<NavTreeNode>,
    metadata: Vec<u8>,
}

impl NavTreeNode {
    pub fn sum_metadata(&self) -> usize {
        let mut metadata_sum: usize = 0;
        for c in &self.children {
            metadata_sum += Self::sum_metadata(c);
        }
        metadata_sum += self.metadata.iter().map(|md| *md as usize).sum::<usize>();
        return metadata_sum;
    }

    pub fn sum_metadata_by_index(&self) -> usize {
        let mut metadata_sum: usize = 0;

        if self.children.len() > 0 {
            for md in &self.metadata {
                if md < &1 {
                    continue;
                }

                if let Some(n) = self.children.get((*md - 1) as usize) {
                    metadata_sum += Self::sum_metadata_by_index(n);
                }
            }
        } else {
            metadata_sum += self.metadata.iter().map(|md| *md as usize).sum::<usize>();
        }

        return metadata_sum;
    }
}

pub struct NavTree<'a, R>
where
    R: Read,
{
    nav_reader: &'a mut NavReader<R>,
}

impl<'a, R> NavTree<'a, R>
where
    R: Read,
{
    pub fn new(nav_reader: &'a mut NavReader<R>) -> Self {
        Self { nav_reader }
    }

    pub fn parse_tree(&mut self) -> Option<NavTreeNode> {
        let num_children = self.nav_reader.next()?;
        let num_metadata = self.nav_reader.next()?;

        let mut children = Vec::new();
        let mut meta_data = Vec::new();

        for _ in 0..num_children {
            let mut sub_tree = NavTree::new(self.nav_reader);
            children.push(sub_tree.parse_tree()?);
        }

        for _ in 0..num_metadata {
            meta_data.push(self.nav_reader.next().unwrap());
        }

        Some(NavTreeNode {
            children,
            metadata: meta_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::string_reader::StringReader;

    use super::*;

    #[test]
    fn can_sum_tree_metadata() {
        let sample_nav_tree = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_owned();
        let mut nav_reader = NavReader::new(StringReader::new(&sample_nav_tree)).unwrap();
        let mut nav_tree = NavTree::new(&mut nav_reader);
        let tree = nav_tree.parse_tree().unwrap();
        assert_eq!(tree.sum_metadata(), 138);
    }

    #[test]
    fn can_sum_tree_metadata_by_index() {
        let sample_nav_tree = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_owned();
        let mut nav_reader = NavReader::new(StringReader::new(&sample_nav_tree)).unwrap();
        let mut nav_tree = NavTree::new(&mut nav_reader);
        let tree = nav_tree.parse_tree().unwrap();
        assert_eq!(tree.sum_metadata_by_index(), 66);
    }
}
