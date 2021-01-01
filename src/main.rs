extern crate serde;
extern crate serde_json;
extern crate reqwest;

use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod parser;
mod result;
mod ip_result;
mod path_result;

fn main() {
    //println!("Analyzer started...");
    let mut result_set = path_result::Result::new();
    let args: Vec<String> = env::args().collect();

    let filename = args[1].to_owned();
    if let Ok(lines) = read_lines(filename) {
        // let mut counter: u8 = 0;
        for line in lines {
            if let Ok(l) = line {
                // counter += 1;
                // if counter > 10 {
                //    return;
                // }
                let result = parser::parse_line(l.to_owned());
                match result {
                    Some(r) => {
                        if !(r.status_code.contains("20") || r.status_code.contains("30")) { // add filter.rs
                            result_set.add_result(r.to_owned());
                        }
                    }
                    None => {
                        //println!("{}", l);
                    }
                }
            }
        }
    }
    let result_json = json!(result_set);
    println!("{}", result_json);
}

// Simple file read iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
