extern crate chariot_palette;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("palette-search")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Find all palette indicies with a given rgb value")
        .arg(Arg::with_name("rgb")
            .long("rgb")
            .value_name("r,g,b")
            .help("The rgb triple to search for")
            .min_values(3)
            .max_values(3)
            .require_delimiter(true)
            .required(true))
        .arg(Arg::with_name("palette-path")
            .long("palette-path")
            .value_name("/path/to/palette.bin")
            .help("The palette to search")
            .required(true))
        .get_matches();

    let (r, g, b) = {
        let mut itr = matches.values_of("rgb").unwrap();

        if let (Some(rs), Some(gs), Some(bs)) = (itr.next(), itr.next(), itr.next()) {
            let r = rs.parse::<u8>().expect("r must be a value between 0 and 255 (inclusive)");
            let g = gs.parse::<u8>().expect("g must be a value between 0 and 255 (inclusive)");
            let b = bs.parse::<u8>().expect("b must be a value between 0 and 255 (inclusive)");
            (r, g, b)
        } else {
            panic!("Argument must be in the form \"r,g,b\"")
        }
    };

    let palette = {
        let arg = matches.value_of("palette-path").unwrap();
        chariot_palette::read_from_file(arg).expect("Failed to read palette!")
    };

    for (idx, color) in palette.iter().enumerate() {
        if color.r == r && color.g == g && color.b == b {
            println!("{}", idx);
        }
    }
}
