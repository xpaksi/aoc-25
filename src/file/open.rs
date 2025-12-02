use std::{
    env::{self},
    fs::File,
    io::Result,
};

pub fn standardize_path(
    filename: String,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    match env::current_dir() {
        Ok(curr_dir) => {
            let path = curr_dir.join("input").join(filename);
            match path.to_str() {
                Some(path_str) => Ok(path_str.to_string()),
                None => Err("Invalid path".into()),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn open_file(path: String) -> Result<File> {
    let file_result = File::open(path);

    match file_result {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}
