use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use gcode_parser::parser::{Commands, parse_command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let path = &args[1];

    println!("Parsing {path}");

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let start = Instant::now();
    let test_results = test_parse(&mut reader)?;
    let duration = start.elapsed();

    println!("{:?}", test_results);
    println!("Time in milliseconds: {}ms", duration.as_millis());

    Ok(())
}

#[derive(Debug)]
struct TestParseResults {
    total_moves: i32,
    total_unreconized_commands: i32,
    total_distance: f64,
}

fn test_parse(
    reader: &mut BufReader<File>,
) -> Result<TestParseResults, Box<dyn std::error::Error>> {
    let mut results = TestParseResults {
        total_moves: 0,
        total_unreconized_commands: 0,
        total_distance: 0.,
    };

    let mut line = String::new();
    let mut line_len = reader.read_line(&mut line)?;
    while line_len > 0 {
        let command = parse_command(&line);

        if let Ok((_remaining, command)) = command {
            if let Commands::G1(params) = command {
                results.total_moves = results.total_moves + 1;

                if let Some(x) = params.x {
                    results.total_distance = results.total_distance + x;
                }
            }
        } else {
            results.total_unreconized_commands = results.total_unreconized_commands + 1;
        }

        line.clear();
        line_len = reader.read_line(&mut line)?;
    }

    Ok(results)
}
