use std::{env, process::exit};
use yukimura::{float32, float64};

// Application
fn main() {
    match env::args().nth(1) {
        Some(command) => {
            match command.as_str() {
                // Commands
                "abc" => abc(false),
                "abc64" => abc(true),
                _ => {
                    print_usage();
                    exit(1);
                }
            }
        },
        None => print_usage()
    }
    exit(0)
}

fn print_usage() {
    println!("Still WIP")
}

// Commands
fn abc(use64: bool) {
    match env::args().nth(4) {
        None => {
            eprintln!("Arguments for a, b and c not set!");
            exit(1)
        },
        Some(cstr) => {
            if use64 {
                // Parse input
                let bstr = env::args().nth(3).unwrap();
                let astr = env::args().nth(2).unwrap();
                if cstr.parse::<f64>().is_err() 
                | bstr.parse::<f64>().is_err()
                | astr.parse::<f64>().is_err() {
                    eprintln!("Arguments for a, b and c must be numbers!");
                    exit(1);
                }
                // Run function
                match float64::abc::abc(
                    astr.parse::<f64>().unwrap(),
                    bstr.parse::<f64>().unwrap(),
                    cstr.parse::<f64>().unwrap()
                ) {
                    Err(e) => eprintln!("{}", e),
                    Ok(x) => {
                        // Print solution
                        if x.x2 == f64::NAN {
                            println!("Only one solution");
                            println!("x: {}", x.x1);
                        } else {
                            print!("x1: {} ", x.x1);
                            println!("x2: {}", x.x2);
                        }
                    }
                }
            } else {
                // Parse input
                let bstr = env::args().nth(3).unwrap();
                let astr = env::args().nth(2).unwrap();
                if cstr.parse::<f32>().is_err() 
                | bstr.parse::<f32>().is_err()
                | astr.parse::<f32>().is_err() {
                    eprintln!("Arguments for a, b and c must be numbers!");
                    exit(1);
                }
                // Run function
                match float32::abc::abc(
                    astr.parse::<f32>().unwrap(),
                    bstr.parse::<f32>().unwrap(),
                    cstr.parse::<f32>().unwrap()
                ) {
                    Err(e) => eprintln!("{}", e),
                    Ok(x) => {
                        // Print solution
                        if x.x2 == f32::NAN {
                            println!("Only one solution");
                            println!("x: {}", x.x1);
                        } else {
                            println!("x1: {} ", x.x1);
                            println!("x2: {}", x.x2);
                        }
                    }
                }
            }
        }
    }
}