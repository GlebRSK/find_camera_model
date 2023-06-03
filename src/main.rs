use clap::{arg, Command};

fn main() {
    let matches = Command::new("read metadata")
        .arg(arg!(--directory <VALUE>).required(true))
        .get_matches();


    let directory = matches.get_one::<String>("directory").expect("required");

    println!("{directory}")
}