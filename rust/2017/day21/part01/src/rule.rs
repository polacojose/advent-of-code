use std::str::FromStr;

use crate::pattern::Pattern;

#[derive(Debug)]
pub struct Rule {
    pub src: Pattern,
    pub dest: Pattern,
}

#[derive(Debug)]
pub struct UnableToParse;

impl FromStr for Rule {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (src, dest) = s.split_once("=>").unwrap();
        Ok(Rule {
            src: src.parse()?,
            dest: dest.parse()?,
        })
    }
}

impl Rule {
    pub fn match_rule(&self, string: impl AsRef<str>) -> bool {
        if let Ok(pattern) = string.as_ref().parse::<Pattern>() {
            return pattern.match_other(&self.src);
        }
        false
    }
}

pub fn transform_pattern(rules: &Vec<Rule>, pattern: &Pattern) -> Option<Pattern> {
    if let Ok(root_patterns) = pattern.root_patterns() {
        let result_patterns = root_patterns
            .into_iter()
            .flat_map(|x| transform_pattern(rules, &x))
            .collect::<Vec<_>>();

        return Some(Pattern::quad(&result_patterns));
    }

    if let Some(rule) = rules.iter().find(|x| x.src.match_other(pattern)) {
        return Some(rule.dest.clone());
    }

    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn demo_rules() -> Vec<Rule> {
        fs::read_to_string("test-input.txt")
            .unwrap()
            .lines()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    #[test]
    fn rule_match_2x2() {
        let rule: Rule = "../.# => ##./#../...".parse().unwrap();
        assert!(rule.match_rule("../.#"));
        assert!(rule.match_rule("#./.."));
        assert!(rule.match_rule(".#/.."));
        assert!(rule.match_rule("../#."));
    }

    #[test]
    fn rule_match_3x3() {
        let rule: Rule = ".#./..#/### => #..#/..../..../#..#".parse().unwrap();
        assert!(rule.match_rule(".#./..#/###"));
        assert!(rule.match_rule("###/#../.#."));
        assert!(rule.match_rule(".##/#.#/..#"));
        assert!(rule.match_rule("#../#.#/##."));
        assert!(rule.match_rule(".#./#../###"));
    }

    #[test]
    fn simple_transform_pattern() {
        let pattern_src = Pattern(vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);
        let pattern_dest = Pattern(vec![
            vec!['#', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '#'],
        ]);

        let rules = demo_rules();
        assert_eq!(transform_pattern(&rules, &pattern_src), Some(pattern_dest));
    }

    #[test]
    fn divided_transform_pattern() {
        let pattern_src = Pattern(vec![
            vec!['#', '.', '#', '#'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '#'],
        ]);
        let pattern_dest = Pattern(vec![
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '#', '#', '#'],
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ]);

        let rules = demo_rules();
        assert_eq!(transform_pattern(&rules, &pattern_src), Some(pattern_dest));
    }

    #[test]
    fn divided_transform_pattern_large() {
        let pattern_src = Pattern(vec![
            vec!['.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ]);

        let pattern_dest = Pattern(vec![
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['#', '#', '#', '#', '#', '#'],
        ]);

        let rules = demo_rules();
        assert_eq!(transform_pattern(&rules, &pattern_src), Some(pattern_dest));
    }
}
