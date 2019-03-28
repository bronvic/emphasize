#  About

Emphasize is a simple CLI program that allows you to emphasize text from stdin or file by text, color or indent

It works the same as grep util, but instead of hiding inappropriate strings it emphasizes matched ones

# Who needs it?
If you are looking some strings in continues flow of logs, you may want to emphasize something you need without
hiding everything else. That was the case I wrote this program for

# How to run
For now, you have to compile it with cargo
```
git clone https://github.com/bronvic/emphasize.git
cd emphasize
cargo build --release
./target/release/emphasize [options] match_value
```

| Command       | Description   |
| ------------- |:-------------|
| -h, --help | Print help menu |
| -f, --file | Path to the input file. If omitted, stdin will be used |
| -i, --indent | Indent before and after match string. Should be in the range of 0-127 |
| -e, --emphasizer | Symbol used to emphasize matched by text. Default symbol is '!'. Works with -t option |
| -c, --color | Color of the matched string. See [list of colors](https://github.com/bronvic/emphasize/blob/master/README.md#list-of-colors) below |
| -t, --text-emphasize | Different modes to emphasize matched string by framing it with added strings and/or modify it's prefix and/or suffix. See [list of modes](https://github.com/bronvic/emphasize/blob/master/README.md#list-of-modes) below |
| -r, --regexp | Search by regexp |
| -C, --without-color | Disables color emphasizing |

## List of colors
You can choose one of this colores: 
* black
* blue
* bright_black
* bright_blue
* bright_cyan
* bright_green
* bright_magenta
* bright_red
* bright_white
* bright_yellow
* cyan
* green
* magenta
* red
* white
* yellow

## List of modes
* none - do not emphasize by text
* frame - inserts string made of emphasize symbols before and after matched string
* prefix - inserts prefix made of one emphasize symbol before matched string
* all - inserts frame, prefix and suffix around matches string

Default emphasize symbol is '!'


# Examples
* ![tail -f /var/log/Xorg.0.log | ./target/debug/emphasize 2.4](https://github.com/bronvic/emphasize/tree/master/content/emph_default.png)
* ![tail -f /var/log/Xorg.0.log | ./target/debug/emphasize -t all -c bright_cyan -i 1 2.4](https://github.com/bronvic/emphasize/tree/master/content/emph_color_mode.png)
* ![tail -f /var/log/Xorg.0.log | ./target/debug/emphasize -t prefix -C -r "([L-N])\w+"](https://github.com/bronvic/emphasize/tree/master/content/emph_regexp.png)
