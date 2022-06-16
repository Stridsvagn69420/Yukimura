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
            let bstr = env::args().nth(3).unwrap();
            let astr = env::args().nth(2).unwrap();
            if use64 {                
                match float64::abc::abc_str(astr.as_str(), bstr.as_str(), cstr.as_str()) {
                    Err(e) => eprintln!("{}", e),
                    Ok(x) => {
                        match x {
                            // Print solution
                            Ok(x) => {
                                if x.x2.is_nan() {
                                    println!("Only one solution");
                                    println!("x: {}", x.x1);
                                } else {
                                    print!("x1: {} ", x.x1);
                                    println!("x2: {}", x.x2);
                                }
                            },
                            Err(_) => {
                                println!("No solution available")
                            }
                        }                        
                    }
                }
            } else {
                match float32::abc::abc_str(astr.as_str(), bstr.as_str(), cstr.as_str()) {
                    Err(e) => eprintln!("{}", e),
                    Ok(x) => {
                        match x {
                            // Print solution
                            Ok(x) => {
                                if x.x2.is_nan() {
                                    println!("Only one solution");
                                    println!("x: {}", x.x1);
                                } else {
                                    print!("x1: {} ", x.x1);
                                    println!("x2: {}", x.x2);
                                }
                            },
                            Err(_) => {
                                println!("No solution available")
                            }
                        }                        
                    }
                }
            }
        }
    }
}