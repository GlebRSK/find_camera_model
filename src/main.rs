extern crate exif;

use clap::{arg, Command};
use exif::{Tag, In};
use walkdir::WalkDir;
use std::collections::HashSet;

pub fn walking(directory: &str) -> Result<HashSet<String>, walkdir::Error> {

    let mut models_set:HashSet<String> = HashSet::new();

    for entry in WalkDir::new(directory) {
        if !entry.as_ref().unwrap().file_type().is_dir() {
            let filename = entry?.path().display().to_string();
            
            match exif_reader(&filename) {
                Ok(e) => {
                    println!("{} {}", filename, e);
                    if e != "" {
                        models_set.insert(e)
                    } else {
                        false
                    }
                    
                },
                Err(_error) => false
            };

        }
    }

    Ok(models_set)

}


pub fn exif_reader(filename: &String) -> Result<String, exif::Error> {
    let exif_tags = [Tag::Model];
    
    for path in [filename] {    
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        
        for &tag in exif_tags.iter() {
            if let Some(field) = exif.get_field(tag, In::PRIMARY) {
                let f = field.display_value().with_unit(&exif).to_string();
                return Ok(f);
            }
        }
    }

    Ok("".to_string())
}


fn main() {
    let matches = Command::new("read metadata")
        .arg(arg!(--directory <VALUE>).required(true))
        .get_matches();


    let directory = matches.get_one::<String>("directory").expect("required");

    let models_set = walking(directory).unwrap();

    println!("{:?}", models_set);
}