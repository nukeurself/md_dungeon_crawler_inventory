use std::io;
use std::fs::{self, DirEntry};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::collections::HashMap;
use toml::Value;

mod item;
pub mod inventory;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_items() {
        let folder = String::from("items/");
        let items = load_items_from_folder(folder);
      
        match items {
            Ok(map) =>  println!("{:?}", map),
            Err(_) => panic!("could not handle items")
        }
    }

    #[test]
    fn key_from_filename() {
        let filename = String::from("sword.toml");
        assert_eq!(get_key_from_filename(&filename), String::from("sword"));

        let filename = String::from("fire_wand.toml");
        assert_eq!(get_key_from_filename(&filename), String::from("fire_wand"));
        
        let filename = String::from("fire.wand.toml");
        assert_eq!(get_key_from_filename(&filename), String::from("fire"));
    }
}


fn load_items_from_folder(dir: String) -> io::Result<HashMap<String, item::Item>> {
    let mut items = HashMap::new();
    let dir = Path::new(&dir);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                let file = fs::read_to_string(path)?;
                let values = file.parse::<Value>().unwrap();

                // add a check of the file extension is toml else return err result

                let filename = entry.file_name().into_string();

                let key = match filename {
                    Ok(value) => get_key_from_filename(&value),
                    Err(_) => {
                        let err = Error::new(ErrorKind::Other, "could not convert filename into string");
                        return Err(err);
                    },
                };
                
                let title = String::from(values["title"].as_str().unwrap());

                items.insert(key, item::Item {
                    title
                });
            } else {
                panic!("Unexpected directory in items folder");
            }
        }
    }

    Ok(items)
}

fn get_key_from_filename(filename: &String) -> String {
    let mut key = String::new();
    for letter in filename.chars() {
        if  letter == '.' {
            break;
        }
        key.push(letter);
    }

    key
}