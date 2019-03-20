use getopts::Options;

pub fn get() -> Options {
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

pub fn print_usage(name: &str, options: Options) {
    let brief = format!("Usage: {} [options] [search substring]\n
        Searches for given substring in standard input and emphasizes it", name);
    print!("{}", options.usage(&brief));
}