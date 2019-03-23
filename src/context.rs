extern crate term;

use getopts::Options;
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;

pub struct Context {
    // It would be great to have such generic iterator here, as commented below
    // We could store there iterator over stdin or file and iterate like this:
    // for line in context.input {

    // Unfortunately, it seems impossible. And it is very very sad.
    // https://stackoverflow.com/questions/55314607/how-to-store-an-iterator-over-stdin-in-a-structure#55314945

    // TODO: become rust guru and resolve this problem

    // pub input: Box<Iterator<Item = io::Result<String>>>

    pub input_filename: String,
    pub match_value: String,

    // emphasize by indent
    pub indent: u8,

    // emphasize by text
    pub emphasizer: char,
    pub frame_mode: FrameMode,

    // emphasize by color
    pub with_color: bool,
    pub text_color: term::color::Color,

    // indicator of problems with argument parsing
    pub just_print_help: bool,
}

custom_derive! {
    #[derive(Debug, EnumDisplay)]
    pub enum FrameMode {
        None,
        Frame,
        Prefix,

        All,
    }
}

fn to_frame_mode(s: &str) -> Result<FrameMode, ()> {
    match s {
            "none" => Ok(FrameMode::None),
            "frame" => Ok(FrameMode::Frame),
            "prefix" => Ok(FrameMode::Prefix),
            "all" => Ok(FrameMode::All),
            _ => Err(()),
        }
}

fn default_context() -> Context {
    Context {
        match_value: "".to_string(),
        input_filename: "".to_string(),
        indent: 0,

        emphasizer: '!',
        frame_mode: FrameMode::None,

        with_color: true,
        text_color: term::color::BRIGHT_RED,

        just_print_help: false,
    }
}

pub fn from_args(args: Vec<String>, options: Options) -> Context {
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

    match matches.opt_str("f") {
        Some(file_name) => match File::open(file_name.clone()) {
            Ok(_) => context.input_filename = file_name,
            Err(_) => context.just_print_help = true,
        },
        None => {},
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

    match matches.opt_str("t") {
        Some(emphasize_type) => {
            match to_frame_mode(&emphasize_type.to_lowercase()) {
                Ok(mode) => context.frame_mode = mode,
                Err(_) => context.just_print_help = true,
            }
        },
        None => {},
    }

    if matches.opt_present("C") {
        context.with_color = false;
    }

    context
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