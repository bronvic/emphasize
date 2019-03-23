extern crate term;
extern crate getopts;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

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

    // TODO: can it be more accurate (without to_vec)?
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
            // indent before
            for _ in 0..context.indent { println!(""); };
            // color settings
            if context.with_color { terminal.fg(context.text_color).unwrap(); };

            // text framing
            match context.frame_mode {
                context::FrameMode::none => {
                    println!("{}", unwrapped_line);
                },
                context::FrameMode::frame => {
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len()));
                    println!("{}", unwrapped_line);
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len()));
                },
                context::FrameMode::prefix => {
                    println!("{}", prefix_line(&unwrapped_line, context.emphasizer));
                },
                context::FrameMode::all => {
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len() + 4));
                    println!("{}", wrapped_line(&unwrapped_line, context.emphasizer));
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len() + 4));
                },
                _ => {
                    panic!("Wrong option {:?} for text emphasize options was not caught!", context.frame_mode);
                }
            }

            // Reset after color change
            // TODO: most likely logs can have there own terminal settings and want not reset, but turn them back. check it
            terminal.reset().unwrap();
            // indent after
            for _ in 0..context.indent { println!(""); };
        }
    }
}

fn emphasize_line(emphasizer: char, length: usize) -> String {
    let mut emphasize_line = String::with_capacity(length);
    for _ in 0..length {
        emphasize_line.push(emphasizer);
    }

    emphasize_line
}

fn prefix_line(line: &str, prefix: char) -> String {
    let mut prefix_line = String::with_capacity(line.len() + 2);
    prefix_line.push(prefix);
    prefix_line.push(' ');
    prefix_line.push_str(line);

    prefix_line
}

fn wrapped_line(line: &str, wrapper: char) -> String {
    let prefix_line = prefix_line(line, wrapper);

    let mut wrapped_line = String::with_capacity(prefix_line.len() + 2);
    wrapped_line.push_str(&prefix_line);
    wrapped_line.push(' ');
    wrapped_line.push(wrapper);

    wrapped_line
}