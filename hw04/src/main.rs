extern crate rand;

use std::process;

pub mod parser;
pub mod rpn;

use parser::read_eval_print_loop;

fn main() {
    if let Err(err) = read_eval_print_loop() {
        match err {
            rpn::Error::Quit => process::exit(0),
            _ => println!("Error: {:?}", err)
        }        
    }
}
