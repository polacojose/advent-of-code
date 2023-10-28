use std::str::FromStr;

use anyhow::anyhow;

use super::diagnostic::{Dir, State, StateCondition, TuringDiagnostic};

#[derive(Debug)]
pub struct UnableToParseStateCondition;
impl FromStr for StateCondition {
    type Err = UnableToParseStateCondition;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let cond_val = match line_value(lines[0])
            .map_err(|_| UnableToParseStateCondition)?
            .as_str()
        {
            "0" => false,
            "1" => true,
            _ => return Err(UnableToParseStateCondition),
        };
        let write_val = match line_value(lines[1])
            .map_err(|_| UnableToParseStateCondition)?
            .as_str()
        {
            "1" => true,
            "0" => false,
            _ => return Err(UnableToParseStateCondition),
        };
        let move_dir = match line_value(lines[2])
            .map_err(|_| UnableToParseStateCondition)?
            .as_str()
        {
            "right" => Dir::Right,
            "left" => Dir::Left,
            _ => return Err(UnableToParseStateCondition),
        };
        let next_state_id = line_value(lines[3]).map_err(|_| UnableToParseStateCondition)?;

        Ok(StateCondition {
            cond_val,
            write_val,
            move_dir,
            next_state_id,
        })
    }
}

#[derive(Debug)]
pub struct UnableToParseState;
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
