use getopts::Options;

pub fn new() -> Options {
    let mut options = Options::new();

    options.optflag("h", "help", "print this help menu");

    options.optopt("f", "file", "path to input file. If omitted, stdin will be used", "");
    options.optopt("i", "indent", "set indent before and after emphasizing", "[0-127]");
    options.optopt("e", "emphasizer", "set emphasize symbol", "Char");
    options.optopt("c", "color", "set color of emphasizing\n
        list of colors: [black, blue, bright_black, bright_blue, bright_cyan, bright_green, bright_magenta, bright_red, bright_white, bright_yellow, cyan, green, magenta, red, white, yellow]", "");

    options.optopt("t", "text-emphasize", "emphasize string by framing it with addition strings and/or modifying it's prefix\n
        Available modes are: [none, frame, prefix, all]", "");

    options.optflag("r", "regexp", "set search by regexp");
    options.optflag("C", "without-color", "disables color emphasizing");

    options
}

pub fn print_usage(name: &String, options: Options) {
    let brief = format!("Usage: {} [options] [search substring]\n
        Searches for given substring in standard input and emphasizes it", name);
    print!("{}", options.usage(&brief));
}