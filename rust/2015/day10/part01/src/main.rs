use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut line = file_string.lines().last().unwrap().to_owned();
    for _ in 0..40 {
        let mut output_line = String::new();
        let mut run_char: Option<char> = None;
        let mut run_length: u32 = 0;

        for byte in line.chars() {
            if let Some(run_char) = run_char {
                if run_char != byte {
                    output_line.push_str(format!("{}{}", run_length, run_char).as_str());
                    run_length = 0;
                }
            }

            run_length += 1;
            run_char = Some(byte);
        }

        output_line.push_str(format!("{}{}", run_length, run_char.unwrap()).as_str());

        println!("{} = {}", line, output_line);
        line = output_line;
    }
    println!("{}", line.len());
}
