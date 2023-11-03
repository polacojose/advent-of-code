use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct DepTree {
    steps: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct UnableToParseDepTree;
impl FromStr for DepTree {
    type Err = UnableToParseDepTree;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();

        let mut steps: HashMap<String, Vec<String>> = Default::default();
        for mut line in lines {
            line = line.trim();
            let parts = line.split_whitespace().collect::<Vec<_>>();

            let parent = parts[parts.len() - 3].to_owned();
            let child = parts[1].to_owned();

            steps
                .entry(parent)
                .and_modify(|v| v.push(child.clone()))
                .or_insert(vec![child.clone()]);
            steps.entry(child).or_insert(vec![]);
        }

        Ok(DepTree { steps })
    }
}

impl DepTree {
    pub fn dependency_order(&self) -> String {
        let mut ordered: Vec<String> = Vec::new();
        let sorted_steps = {
            let mut s = self
                .steps
                .iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect::<Vec<_>>();
            s.sort_unstable_by_key(|x| x.0.to_owned());
            s
        };
        let mut working_list = sorted_steps
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .filter(|(_, v)| v.is_empty())
            .rev()
            .collect::<Vec<_>>();

        let mut remaining_list = sorted_steps
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .filter(|(_, v)| !v.is_empty())
            .collect::<Vec<_>>();

        while let Some(n) = working_list.pop() {
            ordered.push(n.0.to_owned());

            remaining_list = remaining_list
                .into_iter()
                .filter_map(|(k, mut v)| {
                    if let Some(position) = v.iter().position(|c| c == &n.0) {
                        v.remove(position);
                    }
                    if v.is_empty() {
                        working_list.push((k.to_owned(), v.to_owned()));
                        None
                    } else {
                        Some((k, v))
                    }
                })
                .collect::<Vec<_>>();

            // Sorts non dependent jobs by alphabetical order
            working_list.sort_unstable_by_key(|x| x.0.to_owned());
            working_list.reverse();
        }
        ordered.join("")
    }

    pub fn ordered_steps(&self) -> Vec<(String, Vec<String>)> {
        self.dependency_order()
            .chars()
            .map(|c| {
                (
                    c.to_string(),
                    self.steps.get(&c.to_string()).unwrap().to_owned(),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn can_order_steps() {
        let dep_tree: DepTree = "Step F must be finished before step E can begin."
            .parse()
            .unwrap();
        assert!(dep_tree.dependency_order() == "FE");

        let dep_tree: DepTree = "Step B must be finished before step E can begin."
            .parse()
            .unwrap();
        assert!(dep_tree.dependency_order() == "BE");

        let dep_tree: DepTree = "Step C must be finished before step E can begin."
            .parse()
            .unwrap();
        assert!(dep_tree.dependency_order() == "CE");

        let dep_tree: DepTree = r#"
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
        "#
        .parse()
        .unwrap();
        assert!(dep_tree.dependency_order() == "BDFE");

        let dep_tree: DepTree = fs::read_to_string("test-input.txt")
            .unwrap()
            .parse()
            .unwrap();
        assert!(dep_tree.dependency_order() == "CABDFE")
    }
}
