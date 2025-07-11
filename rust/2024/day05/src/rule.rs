use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    first: String,
    second: String,
}

#[derive(Debug)]
pub struct UnableToParse(&'static str);

impl FromStr for Rule {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .trim()
            .split_once("|")
            .ok_or(UnableToParse("Malformed Rule"))?;
        Ok(Rule {
            first: first.trim().to_string(),
            second: second.trim().to_string(),
        })
    }
}

#[derive(Debug)]
pub struct UnableToOrderRules(&'static str);

pub fn pages_in_order(
    rules: HashSet<Rule>,
    pages: impl AsRef<str>,
) -> Result<bool, UnableToOrderRules> {
    let page_graph = order_set(rules.clone(), pages.as_ref().clone()).unwrap();
    Ok(page_graph
        == pages
            .as_ref()
            .split(",")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>())
}

pub fn order_set(
    rules: HashSet<Rule>,
    pages: impl AsRef<str>,
) -> Result<Vec<String>, UnableToOrderRules> {
    let pages: HashSet<String> = pages.as_ref().split(",").map(|s| s.to_string()).collect();
    let rules: HashSet<Rule> = rules
        .iter()
        .filter(|r| pages.contains(&r.first) && pages.contains(&r.second))
        .cloned()
        .collect();
    order_rule_pages(rules)
}

//Implementation of Kahn's algorithm
fn order_rule_pages(mut rules: HashSet<Rule>) -> Result<Vec<String>, UnableToOrderRules> {
    let mut sorted_list = vec![];
    let mut node_set = nodes_without_incoming_edges(&rules)?;

    while let Some(n) = node_set.pop() {
        sorted_list.push(n.clone());

        let (pot_nodes, new_rules): (Vec<String>, HashSet<Rule>) = rules.into_iter().fold(
            (vec![], HashSet::new()),
            |(mut pot_nodes, mut new_rules): (Vec<String>, HashSet<Rule>), r| {
                if r.first == n {
                    pot_nodes.push(r.second);
                } else {
                    new_rules.insert(r);
                }
                (pot_nodes, new_rules)
            },
        );
        rules = new_rules;

        pot_nodes.into_iter().for_each(|m| {
            if rules.iter().all(|r| r.second != m) {
                node_set.push(m);
            }
        });
    }

    if !rules.is_empty() {
        Err(UnableToOrderRules("Rules still left in set"))
    } else {
        Ok(sorted_list)
    }
}

fn nodes_without_incoming_edges(rules: &HashSet<Rule>) -> Result<Vec<String>, UnableToOrderRules> {
    let firsts: HashSet<&String> = rules.iter().map(|r| &r.first).collect();
    let seconds: HashSet<&String> = rules.iter().map(|r| &r.second).collect();

    let diff: Vec<String> = firsts.difference(&seconds).map(|s| *s).cloned().collect();
    if diff.is_empty() {
        Err(UnableToOrderRules("Circular Dependency Detected"))
    } else {
        Ok(diff)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_STR: &str = "47|53
                            97|13
                            97|61
                            97|47
                            75|29
                            61|13
                            75|53
                            29|13
                            97|29
                            53|29
                            61|53
                            97|53
                            61|29
                            47|13
                            75|47
                            97|75
                            47|61
                            75|61
                            47|29
                            75|13
                            53|13";

    macro_rules! eval {
        ($node_list: expr) => {{
            let rules: HashSet<Rule> = TEST_STR
                .trim()
                .lines()
                .map(|s| s.trim().parse::<Rule>().unwrap())
                .collect();

            pages_in_order(rules, $node_list).unwrap()
        }};
    }

    #[test]
    pub fn test_parse() {
        assert!(eval!("75,47,61,53,29") == true);
        assert!(eval!("97,61,53,29,13") == true);
        assert!(eval!("75,29,13") == true);
        assert!(eval!("75,97,47,61,53") == false);
        assert!(eval!("61,13,29") == false);
        assert!(eval!("97,13,75,29,47") == false);
    }
}
