extern crate term;

use crate::options;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Stdout},
    str::FromStr,
};
use term::Terminal;


pub struct Context {
    pub input_filename: String,
    pub match_value: String,
    pub is_regexp: bool,

    // emphasize by indent
    pub indent: u8,

    // emphasize by text
    pub emphasizer: char,
    pub frame_mode: FrameMode,

    // emphasize by color
    pub with_color: bool,
    pub text_color: term::color::Color,

    // indicator of problems with argument parsing or just help printer
    pub print_help: bool,
    pub error_text: String,
}

impl Context {
    fn default() -> Context {
        Context {
            input_filename: "".to_string(),
            match_value: "".to_string(),
            is_regexp: false,

            indent: 0,

            emphasizer: '!',
            frame_mode: FrameMode::None,

            with_color: true,
            text_color: term::color::BRIGHT_RED,

            print_help: false,
            error_text: "".to_string(),
        }
    }

    pub fn from_args(args: &[String]) -> Context {
        let mut context = Context::default();
        let matches = options::new().parse(args).expect("Failed to parse options");

        if matches.opt_present("h") {
            context.print_help = true;
            return context;
        }

        if !matches.free.is_empty() {
            context.match_value = matches.free[0].clone();
        }

        match matches.opt_str("f") {
            Some(file_name) => match File::open(file_name.clone()) {
                Ok(_) => context.input_filename = file_name,
                Err(err) => {
                    context.print_help = true;
                    context.error_text = format!("Wrong input file: {}\n{}", file_name, err.to_string());
                },
            }
            None => {},
        }

        match matches.opt_str("i") {
            Some(indent_str) => match u8::from_str(&indent_str) {
                Ok(indent) => context.indent = indent,
                Err(err) => {
                    context.print_help = true;
                    context.error_text = format!("Wrong indent: {}\n{}", indent_str, err.to_string());
                },
            }
            None => {},
        }

        match matches.opt_str("e") {
            Some(emphasizer_str) => match emphasizer_str.chars().next() {
                Some(emphasizer) => context.emphasizer = emphasizer,
                None => {
                    context.print_help = true;
                    context.error_text = format!("Wrong emphasizer specified: {}", emphasizer_str);
                },
            }
            None => {},
        }

        match matches.opt_str("c") {
            Some(color) => {
                match color_map().get(&color.to_lowercase()) {
                    Some(color) => context.text_color = *color,
                    None => {
                        context.print_help = true;
                        context.error_text = format!("Wrong color: {}", color);
                    },
                }
            },
            None => {},
        }

        match matches.opt_str("t") {
            Some(emphasize_type) => {
                match FrameMode::from_str(&emphasize_type.to_lowercase()) {
                    Ok(mode) => context.frame_mode = mode,
                    Err(_) => {
                        context.print_help = true;
                        context.error_text = format!("Wrong emphasize mode: {}", emphasize_type);
                    },
                }
            },
            None => {},
        }

        if matches.opt_present("r") {
            context.is_regexp = true;

            match Regex::new(&context.match_value) {
                Ok(_) => {},
                Err(err) => {
                    context.print_help = true;
                    context.error_text = format!("Wrong regexp: {}\n{}", context.match_value, err.to_string());
                },
            }
        }

        if matches.opt_present("C") {
            context.with_color = false;
        }

        context
    }

    pub fn get_input(&self) -> Box<dyn Iterator<Item = std::io::Result<String>>> {
        if self.input_filename.is_empty() {
            Box::new(io::stdin().lock().lines())
        } else {
            let file = File::open(&self.input_filename).expect("Can't open file");
            Box::new(BufReader::new(file).lines())
        }
    }

    pub fn match_line(&self, line: &str) -> bool {
        if self.is_regexp {
            Regex::new(&self.match_value).unwrap().is_match(line)
        } else {
            line.contains(&self.match_value)
        }
    }

    pub fn print_emphasized_line(&self, line: &str, terminal: &mut Box<dyn Terminal<Output = Stdout> + Send>) {
        for _ in 0..self.indent { println!(""); };

        if self.with_color { terminal.fg(self.text_color).unwrap(); };

        match self.frame_mode {
            FrameMode::None => {
                println!("{}", line);
            },
            FrameMode::Frame => {
                println!("{}", self.emphasize_line(line.len()));
                println!("{}", line);
                println!("{}", self.emphasize_line(line.len()));
            },
            FrameMode::Prefix => {
                println!("{}", self.prefix_line(line));
            },
            FrameMode::All => {
                println!("{}", self.emphasize_line(line.len() + 4));
                println!("{}", self.wrapped_line(line));
                println!("{}", self.emphasize_line(line.len() + 4));
            },
        }

        terminal.reset().unwrap();
        for _ in 0..self.indent { println!(""); };
    }

    fn emphasize_line(&self, length: usize) -> String {
        let mut emphasized_line = String::new();

        for _ in 0..length {
            emphasized_line.push(self.emphasizer);
        }

        emphasized_line
    }

    pub fn wrapped_line(&self, line: &str) -> String {
        let prefix_line = self.prefix_line(line);

        let mut wrapped_line = String::with_capacity(prefix_line.len() + 2);
        wrapped_line.push_str(&prefix_line);
        wrapped_line.push(' ');
        wrapped_line.push(self.emphasizer);

        wrapped_line
    }

    pub fn prefix_line(&self, line: &str) -> String {
        let mut prefix_line = String::with_capacity(line.len() + 2);
        prefix_line.push(self.emphasizer);
        prefix_line.push(' ');
        prefix_line.push_str(line);

        prefix_line
    }
}

pub enum FrameMode {
    None,
    Frame,
    Prefix,

    All,
}

impl FrameMode {
    fn from_str(s: &str) -> Result<FrameMode, ()> {
        match s {
            "none" => Ok(FrameMode::None),
            "frame" => Ok(FrameMode::Frame),
            "prefix" => Ok(FrameMode::Prefix),
            "all" => Ok(FrameMode::All),
            _ => Err(()),
        }
    }
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