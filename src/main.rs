mod color;
use crate::color::HexColor;
use colored::*;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const SPACING: &str = "    ";

pub fn print_help() {
    println!("{NAME} v{VERSION}");
    println!("{AUTHORS}");
    println!("\n{NAME} linearly interpolates 2 colors to generate a palette.");
    println!("\n\nUSAGE:\n{SPACING}{NAME} [COLOR] [COLOR] [N]");
    println!("\nARGS:\n{SPACING}[COLOR] A color in hexadecimal format (#RRGGBB)");
    println!("{SPACING}[N]     Number of colors in the generated palette");
}

pub fn print_err(err: &str) {
    print_help();
    println!("\n{}", format!("Error: {err}").red());
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut args_iter = args.iter();
    args_iter.next();

    match args.len() - 1 {
        0 => return print_help(),
        3 => {},
        _ => return print_err("Bad argument count")
    }

    let color_a = match HexColor::from_str(args_iter.next().unwrap().as_str()) {
        Ok(x) => x,
        Err(err) => return print_err(err),
    };
    let color_b = match HexColor::from_str(args_iter.next().unwrap().as_str()) {
        Ok(x) => x,
        Err(err) => return print_err(err),
    };

    let n = match args_iter.next().unwrap().parse::<usize>() {
        Ok(x) if x > 2 => x,
        _ => return print_err("Invalid N")
    };

    let palette = HexColor::get_palette(color_a, color_b, n);

    palette.iter().for_each(|color| println!("{color}"));
}
