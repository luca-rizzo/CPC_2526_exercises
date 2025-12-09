use handson3::{course_design, holiday_planning};
use std::error::Error;
use std::fs;

fn run_single_test(
    input_path: &str,
    output_path: &str,
    solver: fn(&str) -> Result<u32, Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)
        .map_err(|_| format!("Impossible to read input: {}", input_path))?;

    let expected: u32 = fs::read_to_string(output_path)?
        .split_whitespace()
        .next()
        .expect("output not present")
        .parse()?;

    let actual = solver(&input)?;

    if actual != expected {
        return Err(format!(
            "❌ Mismatch:\nInput: {}\nExpected: {}\nGot: {}",
            input_path, expected, actual
        )
        .into());
    }
    Ok(())
}

fn main() {
    run_holiday_planning_tests();
    run_course_design_tests();
    println!("\n\nAll tests passed!");
}

fn run_holiday_planning_tests() {
    println!("\n--- Executing holiday_planning tests ---");

    for i in 0..=4 {
        let input = format!("./test_sets/holiday_planning/input{}.txt", i);
        let output = format!("./test_sets/holiday_planning/output{}.txt", i);
        match run_single_test(&input, &output, holiday_planning::solve) {
            Ok(_) => println!("\t✓ holiday_planning test {} OK", i),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_course_design_tests() {
    println!("\n--- Executing course_design tests ---");

    for i in 0..=10 {
        let input = format!("./test_sets/course_design/input{}.txt", i);
        let output = format!("./test_sets/course_design/output{}.txt", i);
        match run_single_test(&input, &output, course_design::solve) {
            Ok(_) => println!("\t✓ course_design test {} OK", i),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
