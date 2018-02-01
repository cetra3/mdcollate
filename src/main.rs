extern crate regex;

use std::path::PathBuf;
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

    let re = Regex::new(r"(?P<image>!)?\[(?P<name>[^\]]*)\]\((?P<href>[^\)]+)\)").unwrap();
    let mut files: Vec<String> = Vec::new();
    let mut processed_files: HashSet<String> = HashSet::new();

    files.push(seed_file.clone());
    processed_files.insert(seed_file.clone());

    while files.len() != 0 {

        let file_name = files.remove(0);
        match resolve_file(file_name.as_ref()) {
            Ok(file) => {
                match read_file(file) {
                    Ok(mut file_contents) => {
                        //We have to clone the file contents as we mutate it later
                        for cap in re.captures_iter(&file_contents.clone()) {
                            let href = cap.name("href").unwrap();
                            let name = cap.name("name").unwrap();
                            let total_match = cap.at(0).unwrap();
                            let mut new_link = resolve_link(file_name.as_ref(), href).unwrap();

                            //Check to see if it's an image
                            let image = match cap.name("image") {
                                Some(value) => value == "!",
                                None => false
                            };

                            match image {
                                true => {
                                    if let Ok(image_file) = resolve_path(&new_link) {
                                        file_contents = file_contents.replace(total_match, &(format!("![{}]({})", name, image_file.display())));
                                    }
                                },
                                false => {

                                    //Replace the links with the inline representation if it's resolvable
                                    match resolve_path(&new_link) {
                                        Ok(file) => {
                                            new_link = String::from(file.to_string_lossy());
                                            file_contents = file_contents.replace(total_match, &(format!("[{}](#{})", name, new_link.replace("/","_"))))
                                        },
                                        Err(_) => ()
                                    }

                                    if !processed_files.contains(&new_link) {
                                        files.push(new_link.clone());
                                        processed_files.insert(new_link.clone());
                                    }
                                }
                            }
                        }

                        //Print out anchor links
                        println!("<p id=\"{}\" class=\"next_file\"></p>\n", file_name.replace("/", "_"));
                        print!("{}", file_contents);
                    },
                    //Ignore file read errors, may be a binary format
                    Err(_) => ()
                }
            },
            //Ignore file resolution errors, may be external
            Err(_) => ()
        }
    }
}


fn read_file(mut file: File) -> Result<String> {
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(_) => Err(Error::new(ErrorKind::InvalidInput,
                      "the file cannot be read"))
    }
}

//Resolves links relative to original files
//Only working on POSIX file systems!
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
    return File::open(resolve_path(search_path)?);
}

fn resolve_path(search_path: &str) -> Result<PathBuf> {

    let path = Path::new(search_path);

    match path.exists(){
        true => {
            let cur_dir = env::current_dir()?;
            let canon_path = path.canonicalize()?;
            let trimmed_path = canon_path.strip_prefix(&cur_dir).unwrap();
            return Ok(trimmed_path.to_path_buf());
        },
        false => {

            if search_path.ends_with(".html") {
                return resolve_path(&search_path[..search_path.len() - 5])
            }

            match search_path.ends_with(".md") {
                true => Err(Error::new(ErrorKind::NotFound,
                              "the file cannot be found")),
                false =>  {
                    return resolve_path(&(format!("{}.md", search_path)));
                }
            }
        }
    }
}
