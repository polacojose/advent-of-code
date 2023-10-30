use ::futures::future::join_all;

pub fn solve_reaction_length(polymer: impl AsRef<str>) -> usize {
    solve_reaction(polymer).len()
}

fn solve_reaction(polymer: impl AsRef<str>) -> String {
    let mut last_polymer_chars = polymer.as_ref().trim().chars().collect::<Vec<_>>();
    loop {
        let mut working_chars = Vec::new();
        let mut i = 1;
        while i < last_polymer_chars.len() {
            if is_opposite_polarity(last_polymer_chars[i - 1], last_polymer_chars[i]) {
                i += 2;
            } else {
                working_chars.push(last_polymer_chars[i - 1]);
                i += 1;
            }
        }

        if i - 1 < last_polymer_chars.len() {
            working_chars.push(last_polymer_chars[i - 1]);
        }

        if working_chars == last_polymer_chars {
            return working_chars.into_iter().collect();
        } else {
            last_polymer_chars = working_chars;
        }
    }
}

fn is_opposite_polarity(a: char, b: char) -> bool {
    if a.to_lowercase().to_string() != b.to_lowercase().to_string() {
        return false;
    }
    if a == b {
        return false;
    }
    return true;
}

fn remove_specified(polymer: impl AsRef<str>, unit: char) -> String {
    let polymer = polymer
        .as_ref()
        .to_owned()
        .replace(&unit.to_uppercase().to_string(), "");
    polymer.replace(&unit.to_lowercase().to_string(), "")
}

pub fn solve_shortest_length_with_removal(polymer: impl AsRef<str>) -> usize {
    let mut shortest = usize::MAX;

    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut futures = Vec::new();

    for alpha in 'a'..'z' {
        let polymer = polymer.as_ref().to_owned();
        futures
            .push(rt.spawn(async move { solve_reaction_length(remove_specified(polymer, alpha)) }));
    }

    rt.block_on(async {
        let mut results = join_all(futures).await.into_iter();
        while let Some(Ok(result)) = results.next() {
            shortest = shortest.min(result);
        }
    });

    return shortest;
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn can_complete_reaction() {
        let polymer = "aA";
        let result = solve_reaction(polymer);
        assert_eq!(result, "");

        let polymer = "abBA";
        let result = solve_reaction(polymer);
        assert_eq!(result, "");

        let polymer = "abAB";
        let result = solve_reaction(polymer);
        assert_eq!(result, "abAB");

        let polymer = "aabAAB";
        let result = solve_reaction(polymer);
        assert_eq!(result, "aabAAB");

        let polymer = fs::read_to_string("test-input.txt").unwrap();
        let result = solve_reaction(polymer);
        assert_eq!(result, "dabCBAcaDA");
    }

    #[test]
    fn should_react_types() {
        assert!(is_opposite_polarity('a', 'A') == true);
        assert!(is_opposite_polarity('A', 'a') == true);
        assert!(is_opposite_polarity('a', 'a') == false);
        assert!(is_opposite_polarity('a', 's') == false);
        assert!(is_opposite_polarity('a', 'S') == false);
    }

    #[test]
    fn can_find_length_width_specified_removal() {
        let polymer = "dabAcCaCBAcCcaDA";
        assert!(solve_reaction_length(remove_specified(polymer, 'a')) == 6);
        assert!(solve_reaction_length(remove_specified(polymer, 'b')) == 8);
        assert!(solve_reaction_length(remove_specified(polymer, 'c')) == 4);
        assert!(solve_reaction_length(remove_specified(polymer, 'd')) == 6);
    }

    #[test]
    fn can_find_shortest_length_with_removal() {
        let polymer = "dabAcCaCBAcCcaDA";
        assert!(solve_shortest_length_with_removal(polymer) == 4);
    }
}
