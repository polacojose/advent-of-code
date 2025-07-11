use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct StateCondition {
    cond_val: bool,
    write_val: bool,
    move_dir: Dir,
    next_state_id: String,
}


#[derive(Debug)]
struct State {
    id: String,
    condition_a: StateCondition,
    condition_b: StateCondition,
}

#[derive(Debug)]
struct UnableToParseState;
impl FromStr for State {
    type Err = UnableToParseState;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let id = line_value(lines[0]).map_err(|_| UnableToParseState)?;
        let mut i = lines.into_iter();
        i.next();

        let conditions: Vec<StateCondition> = i
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|x| x.join("\n"))
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(State {
            id,
            condition_a: conditions[0].clone(),
            condition_b: conditions[1].clone(),
        })
    }
}

//In state A:
//  If the current value is 0:
//    - Write the value 1.
//    - Move one slot to the right.
//    - Continue with state B.
//  If the current value is 1:
//    - Write the value 0.
//    - Move one slot to the left.
//    - Continue with state B.

#[derive(Debug)]
pub struct TuringDiagnostic {
    current_state_id: String,
    steps: u64,
    states: Vec<State>,
}

#[derive(Debug)]
pub struct UnableToParseTuringDiagnostic;
impl FromStr for TuringDiagnostic {
    type Err = UnableToParseTuringDiagnostic;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_chunks = s.split("\n\n").collect::<Vec<_>>();
        let lines = parse_chunks[0].lines().collect::<Vec<_>>();
        let current_state_id = line_value(lines[0]).map_err(|_| UnableToParseTuringDiagnostic)?;
        let steps = lines[1]
            .split_whitespace()
            .nth(5)
            .ok_or(UnableToParseTuringDiagnostic)?
            .parse()
            .map_err(|_| UnableToParseTuringDiagnostic)?;
        let states: Vec<State> = parse_chunks[1..parse_chunks.len()]
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(TuringDiagnostic {
            current_state_id,
            steps,
            states,
        })
    }
}

fn line_value(line: impl AsRef<str>) -> anyhow::Result<String> {
    let last_word = line
        .as_ref()
        .split_whitespace()
        .last()
        .ok_or(anyhow!("UnableToExtractLineValue"))?;
    let mut value = last_word.chars();
    value.next_back();
    Ok(value.as_str().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn turing_diag_parsing() {
        let turing_diagnostic: TuringDiagnostic =
            fs::read_to_string("input.txt").unwrap().parse().unwrap();

        println!("{:#?}", turing_diagnostic);
    }
}
