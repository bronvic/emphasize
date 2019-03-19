extern crate term;
extern crate getopts;

use std::io::{self, BufRead};
use getopts::Options;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let options = construct_options();
    let stdin = io::stdin();
    let mut terminal = term::stdout().unwrap();


    if args.len() < 1 {
        print_usage(&program_name, options);
        return;
    }

    // TODO: more accurate (without to_vec)
    let context = context_from_args(args[1..].to_vec(), options);
    if context.just_print_help {
        // FIXME: can't use options because `options` has type `getopts::Options`, which does not implement the `Copy` trait
        print_usage(&program_name, construct_options());
        return;
    }

    for line in stdin.lock().lines() {
        // TODO: process unwrap
        let unwraped_line = line.unwrap();
        if !unwraped_line.contains(&context.match_value) {
            println!("{}", unwraped_line);
        } else {
            let mut emphasize_framing = String::with_capacity(unwraped_line.len());
            for _ in 0..unwraped_line.len() + if context.frame_string {4} else {0} {
                emphasize_framing.push(context.emphasizer)
            }

            let mut emphasize_line = String::with_capacity(emphasize_framing.len());
            if context.frame_string {
                emphasize_line.push(context.emphasizer);
                emphasize_line.push_str(" ");
                emphasize_line.push_str(&unwraped_line);
                emphasize_line.push_str(" ");
                emphasize_line.push(context.emphasizer);
            } else {
                emphasize_line.push_str(&unwraped_line);
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

struct Context {
    match_value: String,
    indent: u8,

    frame_string: bool,

    emphasizer: char,
    with_emphasizer: bool,

    with_color: bool,
    text_color: term::color::Color,

    just_print_help: bool,
}

fn default_context() -> Context {
    Context {
        match_value: "".to_string(),
        indent: 0,

        with_emphasizer: false,
        emphasizer: '!',

        frame_string: false,

        with_color: true,
        text_color: term::color::BRIGHT_RED,

        just_print_help: false,
    }
}

fn context_from_args(args: Vec<String>, options: Options) -> Context {
    let mut context = default_context();
    let matches = match options.parse(&args) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        context.just_print_help = true;
        return context;
    }

    if !matches.free.is_empty() {
        context.match_value = matches.free[0].clone();
    }

    match matches.opt_str("i") {
        Some(indent_str) => match u8::from_str(&indent_str) {
            Ok(indent) => context.indent = indent,
            Err(_) => context.just_print_help = true,
        },
        None => {},
    }

    match matches.opt_str("e") {
        Some(emphasizer_str) => match emphasizer_str.chars().next() {
            Some(emphasizer) => context.emphasizer = emphasizer,
            None => context.just_print_help = true,
        },
        None => {},
    }

    match matches.opt_str("c") {
        Some(color) => {
            match color_map().get(&color.to_lowercase()) {
                Some(color) => context.text_color = *color,
                None => context.just_print_help = true,
            }
        },
        None => {},
    }

    if matches.opt_present("E") {
        context.with_emphasizer = true;
    }

    if matches.opt_present("F") {
        context.frame_string = true;
    }

    if matches.opt_present("C") {
        context.with_color = false;
    }

    context
}

fn construct_options() -> Options {
    let mut options = Options::new();

    options.optflag("h", "help", "print this help menu");

    options.optopt("i", "indent", "set indent before and after emphasizing", "[0-127]");
    options.optopt("e", "emphasizer", "set emphasize symbol", "Char");
    options.optopt("c", "color", "set color of emphasizing\n
        list of colors: [black, blue, bright_black, bright_blue, bright_cyan, bright_green, bright_magenta, bright_red, bright_white, bright_yellow, cyan, green, magenta, red, white, yellow]", "");

    options.optflag("E", "text-emphasize", "emphasize string by inserting string made from emphasize symbols before and after it");
    options.optflag("F", "framing", "framing of emphasized string");
    options.optflag("C", "without-color", "disables color emphasizing");

    options
}

fn print_usage(name: &str, options: Options) {
    let brief = format!("Usage: {} [options] [search substring]\n
        Searches for given substring in standard input and emphasizes it", name);
    print!("{}", options.usage(&brief));
}

// TODO: can I make it static global and unmutable variable?
fn color_map() -> HashMap<String, term::color::Color> {
    // https://doc.rust-lang.org/1.1.0/term/color/
    let mut map: HashMap<String, term::color::Color> = HashMap::new();

    map.insert("black".to_string(), term::color::BLACK);
    map.insert("blue".to_string(), term::color::BLUE);
    map.insert("bright_black".to_string(), term::color::BRIGHT_BLACK);
    map.insert("bright_blue".to_string(), term::color::BRIGHT_BLUE);
    map.insert("bright_cyan".to_string(), term::color::BRIGHT_CYAN);
    map.insert("bright_green".to_string(), term::color::BRIGHT_GREEN);
    map.insert("bright_magenta".to_string(), term::color::BRIGHT_MAGENTA);
    map.insert("bright_red".to_string(), term::color::BRIGHT_RED);
    map.insert("bright_white".to_string(), term::color::BRIGHT_WHITE);
    map.insert("bright_yellow".to_string(), term::color::BRIGHT_YELLOW);
    map.insert("cyan".to_string(), term::color::CYAN);
    map.insert("green".to_string(), term::color::GREEN);
    map.insert("magenta".to_string(), term::color::MAGENTA);
    map.insert("red".to_string(), term::color::RED);
    map.insert("white".to_string(), term::color::WHITE);
    map.insert("yellow".to_string(), term::color::YELLOW);

    map
}