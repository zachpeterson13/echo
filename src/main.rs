use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// #[command(next_line_help = true)]
pub struct Args {
    #[arg()]
    strings: Vec<String>,

    /// do not output the trailing newline
    #[arg(short)]
    newline: bool,
}

fn main() {
    let args = Args::parse();

    for (index, value) in args.strings.iter().enumerate() {
        if index > 0 {
            print!(" ")
        }
        print!("{}", value)
    }

    if !args.newline {
        println!();
    }
}
