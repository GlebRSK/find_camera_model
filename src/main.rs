extern crate exif;

use clap::{arg, Command};
use exif::{Tag, In};



pub fn exif_reader(filename: &String) -> Result<(), exif::Error> {
    println!("{filename}");
    
    let exif_tags = [Tag::Model];
    
    for path in [filename] {    
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        
        for &tag in exif_tags.iter() {
            if let Some(field) = exif.get_field(tag, In::PRIMARY) {
                let f = field.display_value().with_unit(&exif).to_string();
                println!("{f}");
            }
        }
    }

    Ok(())
}


fn main() {
    let matches = Command::new("read metadata")
        .arg(arg!(--directory <VALUE>).required(true))
        .get_matches();


    let directory = matches.get_one::<String>("directory").expect("required");

    exif_reader(directory);
}