use handson2::{is_there, min_max};
use std::error::Error;
use std::fs;

fn run_single_test(
    input_path: &str,
    output_path: &str,
    solver: fn(&str) -> Result<String, Box<dyn Error>>,
) -> Result<(), String> {
    let input = fs::read_to_string(input_path)
        .map_err(|_| format!("Impossible to read input: {}", input_path))?;

    let expected = fs::read_to_string(output_path)
        .map_err(|_| format!("Impossible to read expected output:  {}", output_path))?;

    let actual = solver(&input).map_err(|e| format!("Error during solve(): {}", e))?;

    if actual != expected {
        return Err(format!(
            "❌ Mismatch:\nInput: {}\nExpected: {}\nGot: {}",
            input_path, expected, actual
        ));
    }
    Ok(())
}
fn main() {
    run_min_max_tests();
    run_is_there_tests();
    println!("\n\nAll tests passed!");
}

fn run_is_there_tests() {
    println!("\n--- Executing is_there tests ---");

    for i in 1..=7 {
        let input = format!("./test_sets/is_there/input{}.txt", i);
        let output = format!("./test_sets/is_there/output{}.txt", i);
        match run_single_test(&input, &output, is_there::solve) {
            Ok(_) => println!("\t✓ is_there test {} OK", i),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_min_max_tests() {
    println!("\n--- Executing min_max tests ---");

    for i in 0..=10 {
        let input = format!("./test_sets/min_max/input{}.txt", i);
        let output = format!("./test_sets/min_max/output{}.txt", i);
        match run_single_test(&input, &output, min_max::solve) {
            Ok(_) => println!("\t✓ min_max test {} OK", i),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
