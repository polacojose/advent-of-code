use std::fs;

use diag::diagnostic::TuringDiagnostic;
use turing::TuringMachine;

mod diag;
pub mod turing;

fn main() {
    let mut td: TuringDiagnostic = fs::read_to_string("input.txt").unwrap().parse().unwrap();

    let mut machine = TuringMachine::default();
    td.run_diagnostic(&mut machine);
}
