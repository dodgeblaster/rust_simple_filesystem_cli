use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(file_name: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn list_directory(path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(path);
    if path.exists() {
        let mut result = String::new();
        for entry in read_dir(path).unwrap() {
            let entry_value = entry.unwrap();
            let is_dir = entry_value.file_type().unwrap().is_dir();
            let file_type = if is_dir { "dir" } else { "file" };

            // cant wait to learn a more elegant way to do this...
            result.push_str(file_type);
            result.push_str("\t");
            result.push_str(entry_value.path().to_str().unwrap());
            result.push_str("\n")
        }
 
        Ok(result)
    } else {
        Ok("Path does not exist".to_string())
    }
}