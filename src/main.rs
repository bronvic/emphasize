extern crate term;
extern crate getopts;
mod context;
mod options;

use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let options = options::get();
    let stdin = io::stdin();
    let mut terminal = term::stdout().unwrap();


    if args.len() < 1 {
        options::print_usage(&program_name, options);
        return;
    }

    // TODO: more accurate (without to_vec)
    let context = context::from_args(args[1..].to_vec(), options);
    if context.just_print_help {
        // FIXME: can't use options because `options` has type `getopts::Options`, which does not implement the `Copy` trait
        options::print_usage(&program_name, options::get());
        return;
    }

    for line in stdin.lock().lines() {
        // TODO: process unwrap
        let unwrapped_line = line.unwrap();
        if !unwrapped_line.contains(&context.match_value) {
            println!("{}", unwrapped_line);
        } else {
            let mut emphasize_framing = String::with_capacity(unwrapped_line.len());
            for _ in 0..unwrapped_line.len() + if context.frame_string {4} else {0} {
                emphasize_framing.push(context.emphasizer)
            }

            let mut emphasize_line = String::with_capacity(emphasize_framing.len());
            if context.frame_string {
                emphasize_line.push(context.emphasizer);
                emphasize_line.push_str(" ");
                emphasize_line.push_str(&unwrapped_line);
                emphasize_line.push_str(" ");
                emphasize_line.push(context.emphasizer);
            } else {
                emphasize_line.push_str(&unwrapped_line);
            }

            if context.with_color {
                terminal.fg(context.text_color).unwrap();
            }

            for _ in 0..context.indent {
                println!("");
            }

            if context.with_emphasizer {
                println!("{}", emphasize_framing);
            }
            println!("{}", emphasize_line);
            if context.with_emphasizer {
                println!("{}", emphasize_framing);
            }
            for _ in 0..context.indent {
                println!("");
            }

            if context.with_color {
                terminal.reset().unwrap();
            }
        }
    }
}
