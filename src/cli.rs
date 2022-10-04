use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify a target file
    #[arg(short, long)]
    file: String,
}

pub fn _init() {
    let args = Args::parse();
    println!("Hello {}!", args.file)
}

// If no args given, process all .txt files in .exe dir
// else process specified file(s)
// error if invalid file
