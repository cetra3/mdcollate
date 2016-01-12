extern crate regex;

use regex::Regex;

use std::env;
use std::path::Path;
use std::io::{Read, Error, ErrorKind,Result};
use std::fs::File;
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => print_col(&args[1]),
        _ => println!("Usage: {} file.md", args[0])
    }
}

//prints a collation of markdown resolving links and inlining them
fn print_col(seed_file: &String) {

    let re = Regex::new(r"(?P<image>!)?\[(?P<name>.*)\]\((?P<href>.*)\)").unwrap();
    let mut files: Vec<String> = Vec::new();
    let mut processed_files: HashSet<String> = HashSet::new();

    files.push(seed_file.clone());
    processed_files.insert(seed_file.clone());

    while files.len() != 0 {

        let file_name = files.remove(0);
        let mut file_contents = read_file(file_name.as_ref());

        //We have to clone the file contents as we mutate it later
        for cap in re.captures_iter(file_contents.clone().as_ref()) {
            let href = cap.name("href").unwrap();
            let name = cap.name("name").unwrap();
            let total_match = cap.at(0).unwrap();
            let new_link = resolve_link(file_name.as_ref(), href).unwrap();

            //Check to see if it's an image
            let image = match cap.name("image") {
                Some(value) => value == "!",
                None => false
            };

            match image {
                true => {
                    file_contents = file_contents.replace(total_match, &(format!("![{}]({})", name, new_link)));
                },
                false => {
                    //Replace the links with the inline representation
                    file_contents = file_contents.replace(total_match, &(format!("[{}](#{})", name, new_link.replace("/","_"))));

                    if !processed_files.contains(&new_link) {
                        files.push(new_link.clone());
                        processed_files.insert(new_link.clone());
                    }
                }
            }
        }

        println!("<a name=\"{}\">&nbsp;</a>", file_name.replace("/", "_"));
        print!("{}", file_contents);
    }
}


fn read_file(filename: &str) -> String {
    let mut file = match resolve_file(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(_) => s
    }
}

//Resolves links relative to original files
fn resolve_link(original_file: &str, link: &str) -> Result<String> {

    let mut filtered_link = String::new();

    //Append the original file's directory if it's not empty
    let original_file_path = format!("{}", Path::new(original_file).parent().unwrap().display());

    if !original_file_path.is_empty() {
        filtered_link.push_str(&*(original_file_path + "/"))
    }

    //Remove ./ in link if at beginning
    match link.starts_with("./") {
        true => filtered_link.push_str(&link[2..]),
        false => filtered_link.push_str(&link)
    };

    Ok(filtered_link)
}

fn resolve_file(search_path: &str) -> Result<File> {
    let path = Path::new(search_path);
    match path.exists(){
        true => File::open(&path),
        false => {
            match search_path.ends_with(".md") {
                true => Err(Error::new(ErrorKind::NotFound,
                              "the file cannot be found")),
                false =>  {
                    return resolve_file(&(format!("{}.md", search_path)));
                }
            }
        }
    }
}
