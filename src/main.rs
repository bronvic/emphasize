extern crate term;
extern crate getopts;
extern crate regex;

mod context;
mod options;

use std::io::{self, BufRead, BufReader};
use std::{env, fs::File};
use regex::Regex;
use context::Context;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut terminal = term::stdout().unwrap();

    let context = Context::from_args(&args[1..]);
    if context.print_help {
        if !context.error_text.is_empty() {
            terminal.fg(term::color::BRIGHT_RED).unwrap();
            print!("{}\n\n", context.error_text);
            terminal.reset().unwrap();
        }

        options::print_usage(&program_name, options::new());
        return;
    }

    // hack due to https://stackoverflow.com/questions/55314607/how-to-store-an-iterator-over-stdin-in-a-structure
    let stdin;
    let mut stdin_lines;
    let mut file_lines;
    let file;
    let input: &mut dyn Iterator<Item = _> = match context.input_filename.is_empty() {
        true => {
            stdin = io::stdin();
            stdin_lines = stdin.lock().lines();
            &mut stdin_lines
        }
        false => {
            file = match File::open(context.input_filename.clone()) {
                Ok(file) => file,
                Err(msg) => panic!("Can't open {}. Reason: {}", context.input_filename, msg),
            };
            file_lines = BufReader::new(file).lines();
            &mut file_lines
        }
    };

    for line in input {
        let unwrapped_line = line.unwrap();
        let matches: bool;

        if context.is_regexp {
            matches = Regex::new(&context.match_value).unwrap().is_match(&unwrapped_line);
        } else {
            matches = unwrapped_line.contains(&context.match_value);
        }

        if !matches {
            println!("{}", unwrapped_line);
        } else {
            // indent before
            for _ in 0..context.indent { println!(""); };
            // color settings
            if context.with_color { terminal.fg(context.text_color).unwrap(); };

            // text framing
            match context.frame_mode {
                context::FrameMode::None => {
                    println!("{}", unwrapped_line);
                },
                context::FrameMode::Frame => {
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len()));
                    println!("{}", unwrapped_line);
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len()));
                },
                context::FrameMode::Prefix => {
                    println!("{}", prefix_line(&unwrapped_line, context.emphasizer));
                },
                context::FrameMode::All => {
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len() + 4));
                    println!("{}", wrapped_line(&unwrapped_line, context.emphasizer));
                    println!("{}", emphasize_line(context.emphasizer, unwrapped_line.len() + 4));
                },
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

fn prefix_line(line: &String, prefix: char) -> String {
    let mut prefix_line = String::with_capacity(line.len() + 2);
    prefix_line.push(prefix);
    prefix_line.push(' ');
    prefix_line.push_str(line);

    prefix_line
}

fn wrapped_line(line: &String, wrapper: char) -> String {
    let prefix_line = prefix_line(line, wrapper);

    let mut wrapped_line = String::with_capacity(prefix_line.len() + 2);
    wrapped_line.push_str(&prefix_line);
    wrapped_line.push(' ');
    wrapped_line.push(wrapper);

    wrapped_line
}
