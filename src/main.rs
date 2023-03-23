mod context;
mod options;

use crate::{context::Context, options::{new as new_options, print_usage}};
use std::{env, io::Stdout};
use term::{stdout, Terminal};


fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let context = Context::from_args(&args[1..]);
    if context.print_help {
        print_help_text(&context.error_text, &program_name);
        return;
    }

    let input = context.get_input();
    process_input(input, &context);
}

fn print_help_text(error_text: &str, program_name: &str) {
    let mut terminal: Box<dyn Terminal<Output = Stdout> + Send> = stdout().unwrap_or_else(|| {
        eprintln!("Failed to initialize terminal.");
        std::process::exit(1);
    });

    if !error_text.is_empty() {
        terminal.fg(term::color::BRIGHT_RED).unwrap();
        print!("{}\n\n", error_text);
        terminal.reset().unwrap();
    }

    print_usage(&program_name.to_string(), new_options());
}

fn process_input(input: Box<dyn Iterator<Item = std::io::Result<String>>>, context: &Context) {
    let mut terminal = term::stdout().unwrap();

    for line in input {
        let unwrapped_line = line.unwrap();
        let matches = context.match_line(&unwrapped_line);

        if !matches {
            println!("{}", unwrapped_line);
        } else {
            context.print_emphasized_line(&unwrapped_line, &mut terminal);
        }
    }
}
